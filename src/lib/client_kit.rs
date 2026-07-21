//! Reusable Near client configurations for testnet and mainnet.
//!
//! # Examples
//!
//! ```no_run
//! use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
//!
//! // Testnet client
//! let near = NEAR_KIT_CLIENT::testnet().build();
//!
//! // Mainnet client
//! let near = NEAR_KIT_CLIENT::mainnet().build();
//!
//! // From env vars: NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY
//! let near = NEAR_KIT_CLIENT::from_env()?.build();
//! ```
// =================================================
use near_kit::{Error, Near, NearBuilder};
use std::env;

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
        self.inner.build()
    }

    /// Build from env vars: `NEAR_NETWORK` (defaults to `testnet`),
    /// `NEAR_ACCOUNT_ID`, `NEAR_PRIVATE_KEY`.
    pub fn from_env() -> Result<Self, near_kit::Error> {
        let network = env::var("NEAR_NETWORK").unwrap_or_else(|_| "testnet".to_string());
        let account_id = env::var("NEAR_ACCOUNT_ID")
            .map_err(|_| Error::Config("NEAR_ACCOUNT_ID env var is required".into()))?;
        let private_key = env::var("NEAR_PRIVATE_KEY")
            .map_err(|_| Error::Config("NEAR_PRIVATE_KEY env var is required".into()))?;

        let me = match network.as_str() {
            "mainnet" => Self::mainnet(),
            "testnet" => Self::testnet(),
            other => {
                return Err(Error::Config(format!(
                    "Unsupported NEAR_NETWORK `{other}` (use `mainnet` or `testnet`)"
                )));
            }
        };
        me.credentials(&private_key, &account_id)
    }
}

// =================================================
// =================================================
// copyright 2026 by sleet.near
