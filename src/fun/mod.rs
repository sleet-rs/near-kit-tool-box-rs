// =================================================
//! Helper functions for interacting with NEAR contracts.
pub mod ft {
    /// View helper: fetch an FT balance via raw JSON args.
    pub mod ft_balance_of_fun_json;
    /// View helper: fetch an FT contract's metadata via raw JSON args.
    pub mod ft_metadata_fun_json;
    /// Change helper: register storage on an FT contract via raw JSON args.
    pub mod ft_storage_deposit_fun_json;
    /// View helper: fetch an FT total supply via raw JSON args.
    pub mod ft_total_supply_fun_json;
    /// Change helper: transfer FT to a receiver and call `ft_transfer_call` via raw JSON args.
    pub mod ft_transfer_call_fun_json;
    /// Change helper: transfer FT to a receiver via raw JSON args.
    pub mod ft_transfer_fun_json;
}
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
pub mod near {
    /// Add an access key to the signer's account using raw JSON args.
    pub mod add_key_fun_json;
    /// Delete the signer's account via raw JSON args.
    pub mod delete_account_fun_json;
    /// Delete an access key from the signer's account using raw JSON args.
    pub mod delete_key_fun_json;
    /// Create a sub-account via the `near` / `testnet` TLD registrar using raw JSON args.
    pub mod near_create_account_fun_json;
    /// Create a sub-account via the `near` / `testnet` TLD registrar using typed contract interface.
    pub mod near_create_account_fun_typed;
    /// Wrap NEAR into wNEAR via raw JSON args.
    pub mod wrap_near_deposit_fun_json;
    /// Unwrap wNEAR back into NEAR via raw JSON args.
    pub mod wrap_near_withdraw_fun_json;
}
// =================================================
