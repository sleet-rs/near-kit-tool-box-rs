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
//! let near = NEAR_KIT_CLIENT::from_env()?.build();
//! # Ok(())
//! # }
//! ```
// =================================================
use near_kit::{Error, Near, NearBuilder};
use std::env;

/// Builder for creating reusable Near clients.
pub struct NEAR_KIT_CLIENT {
    inner: NearBuilder,
}

pub fn print_client_details(near: &Near) {
    let account_id = near
        .try_account_id()
        .map(ToString::to_string)
        .unwrap_or_else(|| "none (read-only)".to_string());
    let mode = if near.signer().is_some() {
        "signing"
    } else {
        "read-only"
    };

    println!("==========================");
    println!("NEAR client details:");
    println!("  Network: {}", near.chain_id());
    println!("  RPC URL: {}", near.rpc_url());
    println!("  Account ID: {account_id}");
    println!("  Mode: {mode}");
    println!("==========================");

    if let Some(public_key) = near.public_key() {
        println!("  Public key: {public_key}");
    }
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

    /// Build from env vars: `NEAR_NETWORK` defaults to `testnet` and may be
    /// a custom RPC URL. `NEAR_CHAIN_ID` is optional. Credentials are optional
    /// and enable signing only when both `NEAR_ACCOUNT_ID` and
    /// `NEAR_PRIVATE_KEY` are set.
    pub fn from_env() -> Result<Self, near_kit::Error> {
        let network = env::var("NEAR_NETWORK").unwrap_or_else(|_| "testnet".to_string());
        let chain_id = env::var("NEAR_CHAIN_ID").ok();
        let mut me = match network.as_str() {
            "mainnet" => Self::mainnet(),
            "testnet" => Self::testnet(),
            rpc_url => Self {
                inner: Near::custom(rpc_url, chain_id.as_deref().unwrap_or("custom")),
            },
        };

        if let Some(chain_id) = chain_id {
            me.inner = me.inner.chain_id(chain_id);
        }

        match (
            env::var("NEAR_ACCOUNT_ID").ok(),
            env::var("NEAR_PRIVATE_KEY").ok(),
        ) {
            (Some(account_id), Some(private_key)) => me.credentials(&private_key, &account_id),
            (Some(_), None) => Err(Error::Config(
                "NEAR_ACCOUNT_ID is set but NEAR_PRIVATE_KEY is missing".into(),
            )),
            (None, Some(_)) => Err(Error::Config(
                "NEAR_PRIVATE_KEY is set but NEAR_ACCOUNT_ID is missing".into(),
            )),
            (None, None) => Ok(me),
        }
    }
}

// =================================================
// =================================================
// copyright 2026 by sleet.near
