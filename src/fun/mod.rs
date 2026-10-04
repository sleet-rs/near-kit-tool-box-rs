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
    pub mod add_key_meta_fun_json;
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
    pub mod view_account_fun_json;
    /// Wrap NEAR into wNEAR via raw JSON args.
    pub mod wrap_near_deposit_fun_json;
    /// Unwrap wNEAR back into NEAR via raw JSON args.
    pub mod wrap_near_withdraw_fun_json;
}
pub mod pumpopoly {
    /// Change helper: perform a Pumpopoly elite move via raw JSON args.
    pub mod pumpopoly_elite_move_fun_json;
    /// Change helper: move a Pumpopoly player via a NEP-366 meta-transaction
    /// (sign + submit pair), with a separate relayer paying gas. Uses raw JSON args.
    pub mod pumpopoly_move_player_meta_fun_json;
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
pub mod rhea_dclv2 {
    /// Change helper: add concentrated liquidity to a DCL v2 pool range via raw JSON args.
    pub mod dclv2_add_liquidity_fun_json;
    /// Change helper: cancel a DCL v2 limit order via raw JSON args.
    pub mod dclv2_cancel_order_fun_json;
    /// Change helper: fund the signer's DCL v2 internal balance via ft_transfer_call with empty msg.
    pub mod dclv2_deposit_fun_json;
    /// View helper: find an account's DCL v2 limit order on one pool point via raw JSON args.
    pub mod dclv2_find_order_fun_json;
    /// View helper: fetch a single DCL v2 limit order by its order id via raw JSON args.
    pub mod dclv2_get_order_fun_json;
    /// View helper: fetch a single DCL v2 liquidity position by its lpt id via raw JSON args.
    pub mod dclv2_get_liquidity_fun_json;
    /// View helper: fetch the per-point liquidity distribution of a DCL v2 pool range via raw JSON args.
    pub mod dclv2_get_liquidity_range_fun_json;
    /// View helper: fetch the enriched view of a single DCL v2 liquidity position via raw JSON args.
    pub mod dclv2_get_liquidity_view_fun_json;
    /// View helper: fetch market-depth snapshots for several DCL v2 pools via raw JSON args.
    pub mod dclv2_get_market_depth_list_fun_json;
    /// View helper: fetch the market-depth snapshot of a DCL v2 pool via raw JSON args.
    pub mod dclv2_get_marketdepth_fun_json;
    /// View helper: fetch the resting limit orders of a DCL v2 pool range via raw JSON args.
    pub mod dclv2_get_pointorder_range_fun_json;
    /// View helper: fetch a single DCL v2 pool by its pool id via raw JSON args.
    pub mod dclv2_get_pool_fun_json;
    /// View helper: fetch several DCL v2 pools by their pool ids via raw JSON args.
    pub mod dclv2_get_pools_fun_json;
    /// View helper: fetch one inner token balance an account holds inside the DCL v2 contract via raw JSON args.
    pub mod dclv2_get_user_asset_fun_json;
    /// View helper: fetch the storage-deposit detail the DCL v2 contract tracks for one user via raw JSON args.
    pub mod dclv2_get_user_storage_detail_fun_json;
    /// View helper: list the still-open DCL v2 limit orders of an account via raw JSON args.
    pub mod dclv2_list_active_orders_fun_json;
    /// View helper: list the filled / cancelled DCL v2 limit orders of an account via raw JSON args.
    pub mod dclv2_list_history_orders_fun_json;
    /// View helper: fetch a paginated slice of DCL v2 liquidity positions of an account via raw JSON args.
    pub mod dclv2_list_liquidities_fun_json;
    /// View helper: fetch a paginated slice of enriched DCL v2 liquidity views of an account via raw JSON args.
    pub mod dclv2_list_liquidity_views_fun_json;
    /// View helper: fetch a paginated slice of DCL v2 pools via raw JSON args.
    pub mod dclv2_list_pools_fun_json;
    /// View helper: list every inner token balance an account holds inside the DCL v2 contract via raw JSON args.
    pub mod dclv2_list_user_assets_fun_json;
    /// View helper: simulate adding liquidity to a DCL v2 pool range via raw JSON args.
    pub mod dclv2_predict_add_liquidity_fun_json;
    /// View helper: simulate removing liquidity from a DCL v2 position via raw JSON args.
    pub mod dclv2_predict_remove_liquidity_fun_json;
    /// View helper: quote an exact-output swap across DCL v2 pools via raw JSON args.
    pub mod dclv2_quote_by_output_fun_json;
    /// View helper: quote an exact-input swap across DCL v2 pools via raw JSON args.
    pub mod dclv2_quote_fun_json;
    /// Change helper: remove concentrated liquidity from a DCL v2 position via raw JSON args.
    pub mod dclv2_remove_liquidity_fun_json;
    /// View helper: fetch the NEP-145 storage balance of an account on the DCL v2 contract via raw JSON args.
    pub mod dclv2_storage_balance_of_fun_json;
    /// Change helper: register storage on the DCL v2 contract via raw JSON args.
    pub mod dclv2_storage_deposit_fun_json;
    /// Change helper: unregister storage on the DCL v2 contract via raw JSON args.
    pub mod dclv2_storage_unregister_fun_json;
    /// Change helper: withdraw excess storage NEAR from the DCL v2 contract via raw JSON args.
    pub mod dclv2_storage_withdraw_fun_json;
    /// Change helper: swap on DCL v2 via ft_transfer_call on the input token via raw JSON args.
    pub mod dclv2_swap_ft_transfer_call_fun_json;
    /// Change helper: withdraw an internal token balance out of the DCL v2 contract via raw JSON args.
    pub mod dclv2_withdraw_asset_fun_json;
}
pub mod rhea_ammv2 {
    /// Change helper: register a new constant-product pool on the rhea_ammv2 / ref-finance DEX via raw JSON args.
    pub mod rhea_ammv2_add_simple_pool_fun_json;
    /// View helper: fetch every LP position an account holds on the rhea_ammv2 / ref-finance DEX via raw JSON args.
    pub mod rhea_ammv2_get_deposits_fun_json;
    /// View helper: fetch the total number of pools registered on the rhea_ammv2 / ref-finance DEX via raw JSON args.
    pub mod rhea_ammv2_get_number_of_pools_fun_json;
    /// View helper: fetch a single pool by its pool id from the rhea_ammv2 / ref-finance DEX via raw JSON args.
    pub mod rhea_ammv2_get_pool_fun_json;
    /// View helper: fetch a paginated slice of pools from the rhea_ammv2 / ref-finance DEX via raw JSON args.
    pub mod rhea_ammv2_get_pools_fun_json;
    /// View helper: simulate a swap on the rhea_ammv2 / ref-finance DEX via raw JSON args.
    pub mod rhea_ammv2_get_return_fun_json;
    /// View helper: fetch the NEP-145 storage balance of an account on the rhea_ammv2 / ref-finance DEX via raw JSON args.
    pub mod rhea_ammv2_storage_balance_of_fun_json;
    /// Change helper: register storage on the rhea_ammv2 / ref-finance DEX for the signer via raw JSON args.
    pub mod rhea_ammv2_storage_deposit_fun_json;
    /// Change helper: perform a swap (or routed multi-hop swap) on the rhea_ammv2 / ref-finance DEX via raw JSON args.
    pub mod rhea_ammv2_swap_fun_json;
    /// Change helper: withdraw a token from the signer's LP position on the rhea_ammv2 / ref-finance DEX via raw JSON args.
    pub mod rhea_ammv2_withdraw_fun_json;
}
// =================================================
