//! Reusable Near client configurations for testnet, mainnet, and custom networks.
//!
//! # Examples
//!
//! ```no_run
//! # fn main() -> Result<(), near_kit::Error> {
//! use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
//!
//! // Testnet client
//! let near = NEAR_KIT_CLIENT::testnet().build();
//!
//! // Mainnet client
//! let near = NEAR_KIT_CLIENT::mainnet().build();
//!
//! // From env vars with optional credentials
//! let near = NEAR_KIT_CLIENT::from_env()?;
//!
//! // Load-balanced view client (round-robin across default endpoints).
//! // Each call picks a Near and either returns it for the caller to use
//! // directly, or runs a proxy method on the picked client.
//! let view = NEAR_KIT_CLIENT::view_balancer_testnet();
//! let near = view.next_near();
//! # Ok(())
//! # }
//! ```
// =================================================
use super::helper::load_env::load_env;
use super::helper::print_client_details::print_client_details;
use super::load_balancer::load_balancing_near::LOAD_BALANCING_NEAR;
use near_kit::{Error, Near, NearBuilder};

/// Builder for creating reusable Near clients.
pub struct NEAR_KIT_CLIENT {
    inner: NearBuilder,
}

impl NEAR_KIT_CLIENT {
    /// Create a new client configured for testnet.
    pub fn testnet() -> Self {
        Self {
            inner: Near::testnet(),
        }
    }

    /// Create a new client configured for mainnet.
    pub fn mainnet() -> Self {
        Self {
            inner: Near::mainnet(),
        }
    }

    /// Add credentials to the client (for signing transactions).
    pub fn credentials(self, private_key: &str, account_id: &str) -> Result<Self, near_kit::Error> {
        let inner = self.inner.credentials(private_key, account_id)?;
        Ok(Self { inner })
    }

    /// Build the Near client instance.
    pub fn build(self) -> Near {
        let near = self.inner.build();
        print_client_details(&near);
        near
    }

    /// Build a client directly from env vars and return the
    /// constructed [`Near`] (prints client details on success).
    ///
    /// Loads `.env` from the current working directory first, so no
    /// `source` / `export` step is needed. Vars already present in the
    /// process environment take precedence over the `.env` file.
    ///
    /// Reads `NEAR_KIT_NETWORK` (defaults to `testnet`, may be a custom
    /// RPC URL), optional `NEAR_KIT_CHAIN_ID`, and optional credentials
    /// via `NEAR_KIT_ACCOUNT_ID` + `NEAR_KIT_PRIVATE_KEY`.
    ///
    /// The `NEAR_KIT_` prefix keeps these vars from colliding with the
    /// generic `NEAR_*` vars that `near-kit` reads on its own.
    pub fn from_env() -> Result<Near, Error> {
        load_env();

        let network = std::env::var("NEAR_KIT_NETWORK").ok();
        let mut chain_id_override = std::env::var("NEAR_KIT_CHAIN_ID").ok();
        let account_id = std::env::var("NEAR_KIT_ACCOUNT_ID").ok();
        let private_key = std::env::var("NEAR_KIT_PRIVATE_KEY").ok();

        let mut builder = match network.as_deref() {
            Some("mainnet") => Near::mainnet(),
            Some("testnet") | None => Near::testnet(),
            Some(url) => {
                let chain_id = chain_id_override
                    .take()
                    .unwrap_or_else(|| "custom".to_string());
                Near::custom(url, chain_id)
            }
        };

        if let Some(id) = chain_id_override {
            builder = builder.chain_id(id);
        }

        match (account_id, private_key) {
            (Some(account), Some(key)) => {
                builder = builder.credentials(&key, account)?;
            }
            (Some(_), None) => {
                return Err(Error::Config(
                    "NEAR_KIT_ACCOUNT_ID is set but NEAR_KIT_PRIVATE_KEY is missing".into(),
                ));
            }
            (None, Some(_)) => {
                return Err(Error::Config(
                    "NEAR_KIT_PRIVATE_KEY is set but NEAR_KIT_ACCOUNT_ID is missing".into(),
                ));
            }
            (None, None) => {
                // Read-only client, no credentials
            }
        }

        let near = builder.build();
        print_client_details(&near);
        Ok(near)
    }

    /// Load-balanced view client over the default public testnet endpoints.
    /// Read-only — do not use for signing.
    pub fn view_balancer_testnet() -> LOAD_BALANCING_NEAR {
        LOAD_BALANCING_NEAR::testnet()
    }

    /// Load-balanced view client over the default public mainnet endpoints.
    /// Read-only — do not use for signing.
    pub fn view_balancer_mainnet() -> LOAD_BALANCING_NEAR {
        LOAD_BALANCING_NEAR::mainnet()
    }

    /// Load-balanced view client over a custom URL list with the given
    /// `chain_id`. Read-only — do not use for signing.
    pub fn view_balancer_with_urls(urls: &[&str], chain_id: &str) -> LOAD_BALANCING_NEAR {
        LOAD_BALANCING_NEAR::new(urls, chain_id)
    }
}

// =================================================
// =================================================
// copyright 2026 by sleet.near
