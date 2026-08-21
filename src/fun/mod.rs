// =================================================
//! Helper functions for interacting with NEAR contracts.
pub mod ft {
    /// View helper: fetch an FT balance via raw JSON args.
    pub mod ft_balance_of_fun_json;
    /// View helper: fetch an FT contract's metadata via raw JSON args.
    pub mod ft_metadata_fun_json;
    /// View helper: fetch the NEP-145 storage balance of an account on an FT contract via raw JSON args.
    pub mod ft_storage_balance_of_fun_json;
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
    /// Add an access key to one or more target accounts via meta-transactions
    /// (NEP-366), with a separate relayer paying gas + storage. Uses raw JSON args.
    pub mod add_key_delegate_fun_json;
    /// Add an access key to the signer's account using raw JSON args.
    pub mod add_key_fun_json;
    /// Delete the signer's account via raw JSON args.
    pub mod delete_account_fun_json;
    /// Delete an access key from the signer's account using raw JSON args.
    pub mod delete_key_fun_json;
    /// Create a sub-account via the `near` / `testnet` TLD registrar using raw JSON args.
    pub mod near_create_account_fun_json;
    pub mod view_account_fun_json;
    /// Create a sub-account via the `near` / `testnet` TLD registrar using typed contract interface.
    pub mod near_create_account_fun_typed;
    /// Wrap NEAR into wNEAR via raw JSON args.
    pub mod wrap_near_deposit_fun_json;
    /// Unwrap wNEAR back into NEAR via raw JSON args.
    pub mod wrap_near_withdraw_fun_json;
}
pub mod pumpopoly {
    /// Change helper: perform a Pumpopoly elite move via raw JSON args.
    pub mod pumpopoly_elite_move_fun_json;
    /// Change helper: move a Pumpopoly player via raw JSON args.
    pub mod pumpopoly_move_player_fun_json;
    /// View helper: fetch a Pumpopoly player's state via raw JSON args.
    pub mod pumpopoly_view_player_fun_json;
    /// View helper: fetch multiple Pumpopoly players' state via raw JSON args.
    pub mod pumpopoly_view_players_fun_json;
}
pub mod pumpopoly_nft {
    /// View helper: count the Pumpopoly NFTs held by an account via raw JSON args.
    pub mod pumpopoly_nft_supply_for_owner_fun_json;
    /// View helper: fetch a single Pumpopoly NFT by token id via raw JSON args.
    pub mod pumpopoly_nft_token_fun_json;
    /// View helper: fetch the Pumpopoly NFT token ids held by accounts via raw JSON args.
    pub mod pumpopoly_nft_token_ids_for_owners_fun_json;
    /// View helper: fetch the Pumpopoly NFTs held by an account via raw JSON args.
    pub mod pumpopoly_nft_tokens_for_owner_fun_json;
    /// Change helper: transfer a Pumpopoly NFT to a receiver contract with a msg via raw JSON args.
    pub mod pumpopoly_nft_transfer_call_fun_json;
    /// Change helper: transfer a Pumpopoly NFT to a receiver via raw JSON args.
    pub mod pumpopoly_nft_transfer_fun_json;
}
pub mod rhea {
    /// Change helper: register a new constant-product pool on the rhea / ref-finance DEX via raw JSON args.
    pub mod rhea_add_simple_pool_fun_json;
    /// View helper: fetch every LP position an account holds on the rhea / ref-finance DEX via raw JSON args.
    pub mod rhea_get_deposits_fun_json;
    /// View helper: fetch the total number of pools registered on the rhea / ref-finance DEX via raw JSON args.
    pub mod rhea_get_number_of_pools_fun_json;
    /// View helper: fetch a single pool by its pool id from the rhea / ref-finance DEX via raw JSON args.
    pub mod rhea_get_pool_fun_json;
    /// View helper: fetch a paginated slice of pools from the rhea / ref-finance DEX via raw JSON args.
    pub mod rhea_get_pools_fun_json;
    /// View helper: simulate a swap on the rhea / ref-finance DEX via raw JSON args.
    pub mod rhea_get_return_fun_json;
    /// View helper: fetch the NEP-145 storage balance of an account on the rhea / ref-finance DEX via raw JSON args.
    pub mod rhea_storage_balance_of_fun_json;
    /// Change helper: register storage on the rhea / ref-finance DEX for the signer via raw JSON args.
    pub mod rhea_storage_deposit_fun_json;
    /// Change helper: perform a swap (or routed multi-hop swap) on the rhea / ref-finance DEX via raw JSON args.
    pub mod rhea_swap_fun_json;
    /// Change helper: withdraw a token from the signer's LP position on the rhea / ref-finance DEX via raw JSON args.
    pub mod rhea_withdraw_fun_json;
}
// =================================================
