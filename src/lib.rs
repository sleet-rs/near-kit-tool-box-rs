//! # near-kit-tool-box
//! 🦀 near kit rs tool box, functions for specific near interactions
//!
//! copyright 2026 by sleet.near
// =================================================
#![allow(non_camel_case_types)]
// =================================================
pub mod lib {
    //! Reusable Near client configurations.
    pub mod client_kit;
    // Contract definitions and typed interfaces for NEAR contracts.
    /// near contracts
    pub mod contract;
    /// near contract method name constants
    pub mod methods;
    /// near contract id constants
    pub mod const_id;
}
// functions
pub mod fun;
// =================================================
// =================================================
// copyright 2026 by sleet.near
