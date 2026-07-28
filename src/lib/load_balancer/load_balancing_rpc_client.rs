// =================================================
// RPC load balancer: distributes read-heavy calls across multiple NEAR endpoints.
// =================================================

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use near_kit::types::{
    AccessKeyListView, AccessKeyView, AccountId, AccountView, BlockEffects, BlockReference,
    BlockView, CryptoHash, EpochValidatorInfo, GasKeyNoncesView, GasPrice, MaintenanceWindow,
    PublicKey, ReceiptToTxResponse, StatusResponse, StateItem, ViewFunctionResult,
    ViewStateResult,
};
use near_kit::{RetryConfig, RpcClient, RpcError};

use super::endpoints::{NEAR_RPC_MAINNET_ENDPOINTS, NEAR_RPC_TESTNET_ENDPOINTS};
use super::strategies::{LoadBalancingStrategy, RoundRobin};

/// Sentinel value indicating no call has been made yet on `last_used_idx`.
const NO_CALL_YET: usize = usize::MAX;

/// RPC load balancer that distributes calls across multiple NEAR endpoints.
///
/// Wraps multiple [`near_kit::RpcClient`]s and picks one per call using a
/// pluggable [`LoadBalancingStrategy`] (round-robin by default). On retryable
/// failures, the next endpoint in the chain is tried before the call returns.
///
/// This is intended for **view** (read-only) operations. Spreading
/// state-changing writes across many nodes makes nonces and replay protection
/// unreliable — for transactions, use the regular [`near_kit::Near`] client.
///
/// Construct directly with [`new`](Self::new), or use the shortcuts on
/// [`NEAR_KIT_CLIENT`](crate::lib::client_kit::NEAR_KIT_CLIENT) for the default
/// testnet / mainnet endpoint lists.
pub struct LOAD_BALANCING_RPC_CLIENT {
    clients: Vec<Arc<RpcClient>>,
    strategy: Arc<dyn LoadBalancingStrategy>,
    failover: bool,
    last_used_idx: AtomicUsize,
}

impl LOAD_BALANCING_RPC_CLIENT {
    /// Create a load-balanced client over the given URLs, using the default
    /// round-robin strategy and failover on retryable errors.
    pub fn new(urls: &[&str]) -> Self {
        let clients = urls
            .iter()
            .map(|url| Arc::new(RpcClient::new(*url)))
            .collect();
        Self {
            clients,
            strategy: Arc::new(RoundRobin::new()),
            failover: true,
            last_used_idx: AtomicUsize::new(NO_CALL_YET),
        }
    }

    /// Create a load-balanced client with a custom retry config applied to
    /// every endpoint. The per-endpoint retries handle transient failures;
    /// failover handles whole-endpoint outages.
    pub fn with_retry_config(urls: &[&str], retry_config: RetryConfig) -> Self {
        let clients = urls
            .iter()
            .map(|url| Arc::new(RpcClient::with_retry_config(*url, retry_config.clone())))
            .collect();
        Self {
            clients,
            strategy: Arc::new(RoundRobin::new()),
            failover: true,
            last_used_idx: AtomicUsize::new(NO_CALL_YET),
        }
    }

    /// Shortcut for a load-balanced client over the public testnet endpoints
    /// in [`NEAR_RPC_TESTNET_ENDPOINTS`].
    pub fn testnet() -> Self {
        Self::new(NEAR_RPC_TESTNET_ENDPOINTS)
    }

    /// Shortcut for a load-balanced client over the public mainnet endpoints
    /// in [`NEAR_RPC_MAINNET_ENDPOINTS`].
    pub fn mainnet() -> Self {
        Self::new(NEAR_RPC_MAINNET_ENDPOINTS)
    }

    /// Replace the load-balancing strategy.
    pub fn with_strategy(mut self, strategy: Arc<dyn LoadBalancingStrategy>) -> Self {
        self.strategy = strategy;
        self
    }

    /// Enable or disable failover. When `true` (default), a retryable failure
    /// on the picked endpoint falls through to the next one before returning.
    pub fn with_failover(mut self, enabled: bool) -> Self {
        self.failover = enabled;
        self
    }

    /// All endpoint URLs in the order they were configured.
    pub fn urls(&self) -> Vec<String> {
        self.clients.iter().map(|c| c.url().to_string()).collect()
    }

    /// The URL the next strategy pick would choose. Mostly useful for logging
    /// and tests — note that the strategy counter is advanced by this call.
    pub fn current_url(&self) -> &str {
        let idx = self.strategy.next(self.clients.len());
        self.clients[idx].url()
    }

    /// The URL that served the last successful call, or `None` if no call has
    /// completed yet. Useful for logging which endpoint actually answered
    /// (especially when failover kicked in).
    pub fn last_used_url(&self) -> Option<&str> {
        let idx = self.last_used_idx.load(Ordering::Relaxed);
        if idx == NO_CALL_YET || idx >= self.clients.len() {
            None
        } else {
            Some(self.clients[idx].url())
        }
    }

    /// Number of configured endpoints.
    pub fn len(&self) -> usize {
        self.clients.len()
    }

    /// `true` if no endpoints are configured.
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    /// Build the ordered chain of endpoints to try for a single call, starting
    /// at the strategy-picked index so the picked endpoint is always tried
    /// first. Each pair is `(index, client)` so the index of the successful
    /// client can be recorded for [`last_used_url`](Self::last_used_url).
    fn client_chain(&self) -> Vec<(usize, Arc<RpcClient>)> {
        let start = self.strategy.next(self.clients.len());
        let len = self.clients.len();
        (0..len)
            .map(move |i| ((start + i) % len, self.clients[(start + i) % len].clone()))
            .collect()
    }

    /// Run an async operation against the picked endpoint, falling through to
    /// the next endpoint on retryable errors when failover is enabled. Records
    /// the index of the client whose call actually returned `Ok`.
    async fn try_with_failover<F, Fut, T>(&self, op: F) -> Result<T, RpcError>
    where
        F: Fn(Arc<RpcClient>) -> Fut,
        Fut: std::future::Future<Output = Result<T, RpcError>>,
    {
        let mut last_err: Option<RpcError> = None;
        for (idx, client) in self.client_chain() {
            match op(client).await {
                Ok(result) => {
                    self.last_used_idx.store(idx, Ordering::Relaxed);
                    return Ok(result);
                }
                Err(e) if self.failover && e.is_retryable() => {
                    last_err = Some(e);
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_err.expect("at least one endpoint must be configured"))
    }

    /// Low-level JSON-RPC call. Distributes across endpoints and fails over on
    /// retryable errors. `params` must be `Clone` so it can be reused across
    /// failover attempts.
    pub async fn call<P, R>(&self, method: &str, params: P) -> Result<R, RpcError>
    where
        P: serde::Serialize + Clone,
        R: serde::de::DeserializeOwned,
    {
        self.try_with_failover(|c| {
            let params = params.clone();
            async move { c.call(method, &params).await }
        })
        .await
    }

    /// View account information.
    pub async fn view_account(
        &self,
        account_id: &AccountId,
        block: BlockReference,
    ) -> Result<AccountView, RpcError> {
        self.try_with_failover(|c| {
            let block = block.clone();
            async move { c.view_account(account_id, block).await }
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
        self.try_with_failover(|c| {
            let block = block.clone();
            async move {
                c.view_access_key(account_id, public_key, block).await
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
        self.try_with_failover(|c| {
            let block = block.clone();
            async move { c.view_access_key_list(account_id, block).await }
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
        self.try_with_failover(|c| {
            let block = block.clone();
            async move {
                c.view_gas_key_nonces(account_id, public_key, block).await
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
        self.try_with_failover(|c| {
            let block = block.clone();
            async move {
                c.view_function(account_id, method_name, args, block).await
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
        self.try_with_failover(|c| {
            let block = block.clone();
            async move {
                c.view_state(account_id, prefix, after_key, limit, block)
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
        self.try_with_failover(|c| {
            let block = block.clone();
            async move {
                c.view_state_all(account_id, prefix, page_size, block).await
            }
        })
        .await
    }

    /// Get block information.
    pub async fn block(&self, block: BlockReference) -> Result<BlockView, RpcError> {
        self.try_with_failover(|c| {
            let block = block.clone();
            async move { c.block(block).await }
        })
        .await
    }

    /// Get all state changes that occurred in a block.
    pub async fn block_effects(
        &self,
        block: BlockReference,
    ) -> Result<BlockEffects, RpcError> {
        self.try_with_failover(|c| {
            let block = block.clone();
            async move { c.block_effects(block).await }
        })
        .await
    }

    /// Get the network's genesis configuration as raw JSON.
    pub async fn genesis_config(&self) -> Result<serde_json::Value, RpcError> {
        self.try_with_failover(|c| async move { c.genesis_config().await })
            .await
    }

    /// Get the upcoming maintenance windows for a validator account.
    pub async fn maintenance_windows(
        &self,
        account_id: &AccountId,
    ) -> Result<Vec<MaintenanceWindow>, RpcError> {
        self.try_with_failover(|c| async move {
            c.maintenance_windows(account_id).await
        })
        .await
    }

    /// Get node status.
    pub async fn status(&self) -> Result<StatusResponse, RpcError> {
        self.try_with_failover(|c| async move { c.status().await })
            .await
    }

    /// Get current gas price.
    pub async fn gas_price(
        &self,
        block_hash: Option<&CryptoHash>,
    ) -> Result<GasPrice, RpcError> {
        self.try_with_failover(|c| async move { c.gas_price(block_hash).await })
            .await
    }

    /// Get validator information for an epoch.
    pub async fn validators(
        &self,
        block: Option<BlockReference>,
    ) -> Result<EpochValidatorInfo, RpcError> {
        self.try_with_failover(|c| {
            let block = block.clone();
            async move { c.validators(block).await }
        })
        .await
    }

    /// Look up the transaction that produced a receipt.
    pub async fn receipt_to_tx(
        &self,
        receipt_id: &CryptoHash,
    ) -> Result<ReceiptToTxResponse, RpcError> {
        self.try_with_failover(|c| async move { c.receipt_to_tx(receipt_id).await })
            .await
    }
}

impl Clone for LOAD_BALANCING_RPC_CLIENT {
    fn clone(&self) -> Self {
        Self {
            clients: self.clients.clone(),
            strategy: self.strategy.clone(),
            failover: self.failover,
            last_used_idx: AtomicUsize::new(
                self.last_used_idx.load(Ordering::Relaxed),
            ),
        }
    }
}

impl std::fmt::Debug for LOAD_BALANCING_RPC_CLIENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LOAD_BALANCING_RPC_CLIENT")
            .field("urls", &self.urls())
            .field("failover", &self.failover)
            .field("last_used_url", &self.last_used_url())
            .finish()
    }
}

// =================================================
// =================================================
