// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use crate::lib::types::rhea_dclv2_user_asset_type::RHEA_DCLV2_USER_ASSETS_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: list every inner token balance an account holds inside
/// the DCL v2 contract using raw JSON args.
///
/// Returns a map of token id to balance, with each balance a decimal
/// string in the token's smallest unit.
pub async fn list_user_assets(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
) -> Result<RHEA_DCLV2_USER_ASSETS_TYPE, Error> {
    let out: RHEA_DCLV2_USER_ASSETS_TYPE = near
        .view::<RHEA_DCLV2_USER_ASSETS_TYPE>(dcl_id, RHEA_DCLV2_METHODS_CONST.list_user_assets)
        .args(json!({ "account_id": account_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
