use serde::Deserialize;
use std::collections::HashMap;
// =================================================
/// NEP-177 token metadata as returned inside a `Token` by the
/// nft.pumpopoly.near view methods.
///
/// Every field past `title` is nullable on this contract — land and bond
/// NFTs only populate a subset.
#[derive(Debug, Clone, Deserialize)]
pub struct NFT_TOKEN_METADATA_RESPONSE_TYPE {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub media_hash: Option<String>,
    pub copies: Option<u64>,
    pub issued_at: Option<String>,
    pub expires_at: Option<String>,
    pub starts_at: Option<String>,
    pub updated_at: Option<String>,
    pub extra: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
}
// =================================================
/// NEP-171 token returned by `nft_token`, `nft_tokens` and
/// `nft_tokens_for_owner`.
///
/// `approved_account_ids` is the NEP-178 approval map of
/// `account_id -> approval_id`; it is an empty map when nothing is approved.
#[derive(Debug, Clone, Deserialize)]
pub struct NFT_TOKEN_RESPONSE_TYPE {
    pub token_id: String,
    pub owner_id: String,
    pub metadata: Option<NFT_TOKEN_METADATA_RESPONSE_TYPE>,
    pub approved_account_ids: Option<HashMap<String, u64>>,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
