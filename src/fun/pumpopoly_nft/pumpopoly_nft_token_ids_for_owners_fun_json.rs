// use near_kit::*;
use crate::lib::methods::methods_pumpopoly_nft::PUMPOPOLY_NFT_METHODS_CONST;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
use std::collections::HashMap;
// =================================================
/// View helper: fetch just the token ids held by each of `account_ids`,
/// using raw JSON args.
///
/// Returns a map of `account_id -> token_ids`. This is the cheap way to
/// enumerate a large holder — unlike `nft_tokens_for_owner` it skips
/// metadata and is not paginated, so a full 100+ token wallet comes back
/// in one call.
pub async fn nft_token_ids_for_owners(
    near: &Near,
    nft_contract_id: &str,
    account_ids: &[&str],
) -> Result<HashMap<String, Vec<String>>, Error> {
    let accounts: Vec<AccountId> = account_ids
        .iter()
        .map(|a| a.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let token_ids: HashMap<String, Vec<String>> = near
        .view::<HashMap<String, Vec<String>>>(
            nft_contract_id,
            PUMPOPOLY_NFT_METHODS_CONST.nft_token_ids_for_owners,
        )
        .args(json!({ "account_ids": accounts }))
        .await?;
    Ok(token_ids)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
