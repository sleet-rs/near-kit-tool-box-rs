//! # near-kit-tool-box
//! 🦀 near kit rs tool box, functions for specific near interactions
//!
//! copyright 2026 by sleet.near
// =================================================
#![allow(non_camel_case_types)]
// =================================================
pub mod lib {
    //! Contract definitions and typed interfaces for NEAR contracts.
    /// near greetng smart contract trait
    pub mod contract_greeting;
}
pub mod fun {
    //! Helper functions for interacting with NEAR contracts.
    /// a get greeting function for a near greeting contract.
    pub mod greeting_get_fun;
    /// a set greeting function for a near greeting contract.
    pub mod greeting_set_fun_json;
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
