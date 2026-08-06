// use near_kit::*;
use crate::lib::methods::methods_pumpopoly_nft::PUMPOPOLY_NFT_METHODS_CONST;
use crate::lib::types::nft_token_response_type::NFT_TOKEN_RESPONSE_TYPE;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// View helper: list the Pumpopoly NFTs held by `account_id`, with full
/// token metadata, using raw JSON args.
///
/// `from_index` is a `U128` JSON string offset (e.g. `"0"`) and `limit`
/// caps the page size. Both are optional — passing `None` lets the
/// contract apply its own defaults. Keep `limit` modest (~50) since each
/// token carries a full metadata blob and the view call has a gas budget.
pub async fn nft_tokens_for_owner(
    near: &Near,
    nft_contract_id: &str,
    account_id: &str,
    from_index: Option<&str>,
    limit: Option<u64>,
) -> Result<Vec<NFT_TOKEN_RESPONSE_TYPE>, Error> {
    let account: AccountId = account_id.parse()?;
    let tokens: Vec<NFT_TOKEN_RESPONSE_TYPE> = near
        .view::<Vec<NFT_TOKEN_RESPONSE_TYPE>>(
            nft_contract_id,
            PUMPOPOLY_NFT_METHODS_CONST.nft_tokens_for_owner,
        )
        .args(json!({
            "account_id": account,
            "from_index": from_index,
            "limit": limit,
        }))
        .await?;
    Ok(tokens)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
