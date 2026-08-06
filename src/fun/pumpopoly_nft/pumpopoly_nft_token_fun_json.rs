// use near_kit::*;
use crate::lib::methods::methods_pumpopoly_nft::PUMPOPOLY_NFT_METHODS_CONST;
use crate::lib::types::nft_token_response_type::NFT_TOKEN_RESPONSE_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch a single Pumpopoly NFT by `token_id`, using raw
/// JSON args.
///
/// Returns `None` when the token does not exist. Token ids on this
/// contract are human readable and contain spaces
/// (e.g. `"Bond 10-849-1"`), so quote them when passing from a shell.
pub async fn nft_token(
    near: &Near,
    nft_contract_id: &str,
    token_id: &str,
) -> Result<Option<NFT_TOKEN_RESPONSE_TYPE>, Error> {
    let token: Option<NFT_TOKEN_RESPONSE_TYPE> = near
        .view::<Option<NFT_TOKEN_RESPONSE_TYPE>>(
            nft_contract_id,
            PUMPOPOLY_NFT_METHODS_CONST.nft_token,
        )
        .args(json!({ "token_id": token_id }))
        .await?;
    Ok(token)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
