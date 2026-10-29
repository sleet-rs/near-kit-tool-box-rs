// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use crate::lib::types::rhea_dclv2_user_asset_type::RHEA_DCLV2_USER_ASSET_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch one inner token balance an account holds inside the
/// DCL v2 contract using raw JSON args.
///
/// `token_id` is the underlying FT account id. The contract answers with
/// a bare decimal string in the token's smallest unit, for example
/// `"0"`.
pub async fn get_user_asset(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
    token_id: &str,
) -> Result<RHEA_DCLV2_USER_ASSET_TYPE, Error> {
    let out: RHEA_DCLV2_USER_ASSET_TYPE = near
        .view::<RHEA_DCLV2_USER_ASSET_TYPE>(dcl_id, RHEA_DCLV2_METHODS_CONST.get_user_asset)
        .args(json!({ "account_id": account_id, "token_id": token_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
