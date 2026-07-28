// =================================================
// Near load balancer: distributes read-heavy calls across multiple configured
// `Near` clients. Each call picks one client (round-robin by default) and
// either returns it for the caller to use directly, or delegates to a proxy
// method on the picked client.
// =================================================

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use near_kit::types::{
    AccessKeyListView, AccessKeyView, AccountId, AccountView, BlockEffects, BlockReference,
    BlockView, CryptoHash, EpochValidatorInfo, GasKeyNoncesView, GasPrice, MaintenanceWindow,
    PublicKey, ReceiptToTxResponse, StatusResponse, StateItem, ViewFunctionResult,
    ViewStateResult,
};
use near_kit::{Near, RetryConfig, RpcError};

use super::endpoints::{NEAR_RPC_MAINNET_ENDPOINTS, NEAR_RPC_TESTNET_ENDPOINTS};
use super::strategies::{LoadBalancingStrategy, RoundRobin};

/// Sentinel value indicating no call has been made yet on `last_used_idx`.
const NO_CALL_YET: usize = usize::MAX;

/// Load balancer that distributes calls across multiple configured [`Near`]
/// clients.
///
/// Each endpoint URL is wrapped in a fully-built [`Near`] (via
/// [`Near::custom`] for the given `chain_id`), so callers can either:
///
/// - **Pick a `Near`** with [`next_near`](Self::next_near) and pass it to any
///   function expecting `&Near` (the picked client is `Clone`).
/// - **Use a proxy method** like [`status`](Self::status) or
///   [`view_function`](Self::view_function) — internally picks a client, runs
///   the call on it, and records which endpoint served it (see
///   [`last_used_url`](Self::last_used_url)).
///
/// This is intended for **view** (read-only) operations. Spreading
/// state-changing writes across many nodes makes nonces and replay protection
/// unreliable — for transactions, use a single regular [`Near`] client.
///
/// Construct directly with [`new`](Self::new), or use the shortcuts on
/// [`NEAR_KIT_CLIENT`](crate::lib::client_kit::NEAR_KIT_CLIENT) for the default
/// testnet / mainnet endpoint lists.
pub struct LOAD_BALANCING_NEAR {
    nears: Vec<Near>,
    strategy: Arc<dyn LoadBalancingStrategy>,
    failover: bool,
    last_used_idx: AtomicUsize,
}

impl LOAD_BALANCING_NEAR {
    /// Create a load-balanced client over the given URLs, using the default
    /// round-robin strategy and failover on retryable errors. Each URL is
    /// wrapped in a [`Near::custom`] client with the given `chain_id`.
    pub fn new(urls: &[&str], chain_id: &str) -> Self {
        let nears = urls
            .iter()
            .map(|url| Near::custom(*url, chain_id).build())
            .collect();
        Self {
            nears,
            strategy: Arc::new(RoundRobin::new()),
            failover: true,
            last_used_idx: AtomicUsize::new(NO_CALL_YET),
        }
    }

    /// Create a load-balanced client with a custom retry config applied to
    /// every endpoint's [`Near`]. The per-endpoint retries handle transient
    /// failures; failover handles whole-endpoint outages.
    pub fn with_retry_config(
        urls: &[&str],
        chain_id: &str,
        retry_config: RetryConfig,
    ) -> Self {
        let nears = urls
            .iter()
            .map(|url| {
                Near::custom(*url, chain_id)
                    .retry_config(retry_config.clone())
                    .build()
            })
            .collect();
        Self {
            nears,
            strategy: Arc::new(RoundRobin::new()),
            failover: true,
            last_used_idx: AtomicUsize::new(NO_CALL_YET),
        }
    }

    /// Shortcut for a load-balanced client over the public testnet endpoints
    /// in [`NEAR_RPC_TESTNET_ENDPOINTS`].
    pub fn testnet() -> Self {
        Self::new(NEAR_RPC_TESTNET_ENDPOINTS, "testnet")
    }

    /// Shortcut for a load-balanced client over the public mainnet endpoints
    /// in [`NEAR_RPC_MAINNET_ENDPOINTS`].
    pub fn mainnet() -> Self {
        Self::new(NEAR_RPC_MAINNET_ENDPOINTS, "mainnet")
    }

    /// Replace the load-balancing strategy.
    pub fn with_strategy(mut self, strategy: Arc<dyn LoadBalancingStrategy>) -> Self {
        self.strategy = strategy;
        self
    }

    /// Enable or disable failover. When `true` (default), any failure on the
    /// picked endpoint falls through to the next one before returning. Note
    /// that this is broader than `RpcError::is_retryable()` — a 403 from a
    /// misconfigured node is classified as non-retryable by near-kit, but
    /// for a load balancer the right move is to try the next endpoint, not
    /// to give up.
    pub fn with_failover(mut self, enabled: bool) -> Self {
        self.failover = enabled;
        self
    }

    /// Pick the next [`Near`] (round-robin or strategy-picked), record it as
    /// the last-used endpoint, and return a clone. The returned client is a
    /// fully-formed `Near` — pass it as `&Near` to any function that accepts
    /// a regular near-kit client.
    pub fn next_near(&self) -> Near {
        let idx = self.pick_index();
        self.nears[idx].clone()
    }

    /// Peek the next [`Near`] without advancing the strategy counter. Mostly
    /// useful for logging and tests.
    pub fn current_near(&self) -> &Near {
        let idx = self.strategy.next(self.nears.len());
        &self.nears[idx]
    }

    /// All endpoint URLs in the order they were configured.
    pub fn urls(&self) -> Vec<String> {
        self.nears.iter().map(|n| n.rpc_url().to_string()).collect()
    }

    /// The URL the next strategy pick would choose. Mostly useful for logging
    /// and tests — note that the strategy counter is advanced by this call.
    pub fn current_url(&self) -> &str {
        self.current_near().rpc_url()
    }

    /// The URL that served the last successful call, or `None` if no call has
    /// completed yet. Useful for logging which endpoint actually answered
    /// (especially when failover kicked in).
    pub fn last_used_url(&self) -> Option<&str> {
        let idx = self.last_used_idx.load(Ordering::Relaxed);
        if idx == NO_CALL_YET || idx >= self.nears.len() {
            None
        } else {
            Some(self.nears[idx].rpc_url())
        }
    }

    /// Number of configured endpoints.
    pub fn len(&self) -> usize {
        self.nears.len()
    }

    /// `true` if no endpoints are configured.
    pub fn is_empty(&self) -> bool {
        self.nears.is_empty()
    }

    /// Pick an index via the strategy and record it as `last_used_idx`. Used
    /// by all proxy methods below.
    fn pick_index(&self) -> usize {
        let idx = self.strategy.next(self.nears.len());
        self.last_used_idx.store(idx, Ordering::Relaxed);
        idx
    }

    /// Build the ordered chain of (index, near) pairs to try for a single
    /// call. Starts at the strategy-picked index so the picked endpoint is
    /// always tried first.
    fn near_chain(&self) -> Vec<(usize, Near)> {
        let start = self.strategy.next(self.nears.len());
        let len = self.nears.len();
        (0..len)
            .map(move |i| ((start + i) % len, self.nears[(start + i) % len].clone()))
            .collect()
    }

    /// Run an async operation against the picked endpoint, falling through to
    /// the next endpoint on any error when failover is enabled. Records the
    /// index of the client whose call actually returned `Ok`.
    async fn try_with_failover<F, Fut, T>(&self, op: F) -> Result<T, RpcError>
    where
        F: Fn(Near) -> Fut,
        Fut: std::future::Future<Output = Result<T, RpcError>>,
    {
        let mut last_err: Option<RpcError> = None;
        for (idx, near) in self.near_chain() {
            match op(near).await {
                Ok(result) => {
                    self.last_used_idx.store(idx, Ordering::Relaxed);
                    return Ok(result);
                }
                Err(e) if self.failover => {
                    last_err = Some(e);
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_err.expect("at least one endpoint must be configured"))
    }

    // -------------------------------------------------------
    // Proxy methods — each picks a Near, delegates to its rpc()
    // -------------------------------------------------------

    /// View account information.
    pub async fn view_account(
        &self,
        account_id: &AccountId,
        block: BlockReference,
    ) -> Result<AccountView, RpcError> {
        self.try_with_failover(|near| {
            let block = block.clone();
            let account_id = account_id.clone();
            async move { near.rpc().view_account(&account_id, block).await }
        })
        .await
    }

    /// View access key information.
    pub async fn view_access_key(
        &self,
        account_id: &AccountId,
        public_key: &PublicKey,
        block: BlockReference,
    ) -> Result<AccessKeyView, RpcError> {
        self.try_with_failover(|near| {
            let block = block.clone();
            let account_id = account_id.clone();
            let public_key = public_key.clone();
            async move {
                near.rpc()
                    .view_access_key(&account_id, &public_key, block)
                    .await
            }
        })
        .await
    }

    /// View all access keys for an account.
    pub async fn view_access_key_list(
        &self,
        account_id: &AccountId,
        block: BlockReference,
    ) -> Result<AccessKeyListView, RpcError> {
        self.try_with_failover(|near| {
            let block = block.clone();
            let account_id = account_id.clone();
            async move { near.rpc().view_access_key_list(&account_id, block).await }
        })
        .await
    }

    /// View the parallel nonces assigned to a gas key.
    pub async fn view_gas_key_nonces(
        &self,
        account_id: &AccountId,
        public_key: &PublicKey,
        block: BlockReference,
    ) -> Result<GasKeyNoncesView, RpcError> {
        self.try_with_failover(|near| {
            let block = block.clone();
            let account_id = account_id.clone();
            let public_key = public_key.clone();
            async move {
                near.rpc()
                    .view_gas_key_nonces(&account_id, &public_key, block)
                    .await
            }
        })
        .await
    }

    /// Call a view function on a contract.
    pub async fn view_function(
        &self,
        account_id: &AccountId,
        method_name: &str,
        args: &[u8],
        block: BlockReference,
    ) -> Result<ViewFunctionResult, RpcError> {
        self.try_with_failover(|near| {
            let block = block.clone();
            let account_id = account_id.clone();
            let args = args.to_vec();
            let method_name = method_name.to_string();
            async move {
                near.rpc()
                    .view_function(&account_id, &method_name, &args, block)
                    .await
            }
        })
        .await
    }

    /// View a single page of a contract's state.
    pub async fn view_state(
        &self,
        account_id: &AccountId,
        prefix: &[u8],
        after_key: Option<&[u8]>,
        limit: Option<u32>,
        block: BlockReference,
    ) -> Result<ViewStateResult, RpcError> {
        self.try_with_failover(|near| {
            let block = block.clone();
            let account_id = account_id.clone();
            let prefix = prefix.to_vec();
            let after_key = after_key.map(|k| k.to_vec());
            async move {
                near.rpc()
                    .view_state(&account_id, &prefix, after_key.as_deref(), limit, block)
                    .await
            }
        })
        .await
    }

    /// Read a contract's entire state, transparently following pagination.
    pub async fn view_state_all(
        &self,
        account_id: &AccountId,
        prefix: &[u8],
        page_size: u32,
        block: BlockReference,
    ) -> Result<Vec<StateItem>, RpcError> {
        self.try_with_failover(|near| {
            let block = block.clone();
            let account_id = account_id.clone();
            let prefix = prefix.to_vec();
            async move {
                near.rpc()
                    .view_state_all(&account_id, &prefix, page_size, block)
                    .await
            }
        })
        .await
    }

    /// Get block information.
    pub async fn block(&self, block: BlockReference) -> Result<BlockView, RpcError> {
        self.try_with_failover(|near| {
            let block = block.clone();
            async move { near.rpc().block(block).await }
        })
        .await
    }

    /// Get all state changes that occurred in a block.
    pub async fn block_effects(
        &self,
        block: BlockReference,
    ) -> Result<BlockEffects, RpcError> {
        self.try_with_failover(|near| {
            let block = block.clone();
            async move { near.rpc().block_effects(block).await }
        })
        .await
    }

    /// Get the network's genesis configuration as raw JSON.
    pub async fn genesis_config(&self) -> Result<serde_json::Value, RpcError> {
        self.try_with_failover(|near| async move { near.rpc().genesis_config().await })
            .await
    }

    /// Get the upcoming maintenance windows for a validator account.
    pub async fn maintenance_windows(
        &self,
        account_id: &AccountId,
    ) -> Result<Vec<MaintenanceWindow>, RpcError> {
        self.try_with_failover(|near| {
            let account_id = account_id.clone();
            async move { near.rpc().maintenance_windows(&account_id).await }
        })
        .await
    }

    /// Get node status.
    pub async fn status(&self) -> Result<StatusResponse, RpcError> {
        self.try_with_failover(|near| async move { near.rpc().status().await })
            .await
    }

    /// Get current gas price.
    pub async fn gas_price(
        &self,
        block_hash: Option<&CryptoHash>,
    ) -> Result<GasPrice, RpcError> {
        self.try_with_failover(|near| {
            let block_hash = block_hash.cloned();
            async move { near.rpc().gas_price(block_hash.as_ref()).await }
        })
        .await
    }

    /// Get validator information for an epoch.
    pub async fn validators(
        &self,
        block: Option<BlockReference>,
    ) -> Result<EpochValidatorInfo, RpcError> {
        self.try_with_failover(|near| {
            let block = block.clone();
            async move { near.rpc().validators(block).await }
        })
        .await
    }

    /// Look up the transaction that produced a receipt.
    pub async fn receipt_to_tx(
        &self,
        receipt_id: &CryptoHash,
    ) -> Result<ReceiptToTxResponse, RpcError> {
        self.try_with_failover(|near| {
            let receipt_id = *receipt_id;
            async move { near.rpc().receipt_to_tx(&receipt_id).await }
        })
        .await
    }
}

impl Clone for LOAD_BALANCING_NEAR {
    fn clone(&self) -> Self {
        Self {
            nears: self.nears.clone(),
            strategy: self.strategy.clone(),
            failover: self.failover,
            last_used_idx: AtomicUsize::new(self.last_used_idx.load(Ordering::Relaxed)),
        }
    }
}

impl std::fmt::Debug for LOAD_BALANCING_NEAR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LOAD_BALANCING_NEAR")
            .field("urls", &self.urls())
            .field("failover", &self.failover)
            .field("last_used_url", &self.last_used_url())
            .finish()
    }
}

// =================================================
// =================================================