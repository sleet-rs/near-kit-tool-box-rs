// =================================================
//! Helper functions for interacting with NEAR contracts.
pub mod greeting {
    /// a get greeting function for a near greeting contract using typed contract interface.
    pub mod greeting_get_fun;
    /// a get greeting function for a near greeting contract using raw JSON args.
    pub mod greeting_get_fun_json;
    /// a set greeting function for a near greeting contract using typed contract interface.
    pub mod greeting_set_fun;
    /// a set greeting function for a near greeting contract using raw JSON args.
    pub mod greeting_set_fun_json;
}
// =================================================
