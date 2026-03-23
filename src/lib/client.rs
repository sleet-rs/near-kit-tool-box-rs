//! Reusable Near client configurations for testnet and mainnet.
//!
//! # Examples
//!
//! ```no_run
//! use near_kit_tool_box::lib::client::NearClient;
//!
//! // Testnet client
//! let near = NearClient::testnet().build();
//!
//! // Mainnet client
//! let near = NearClient::mainnet().build();
//! ```
// =================================================
use near_kit::{Near, NearBuilder};

/// Builder for creating reusable Near clients.
pub struct NearClient {
    inner: NearBuilder,
}

impl NearClient {
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
}

// =================================================
// =================================================
// copyright 2026 by sleet.near
