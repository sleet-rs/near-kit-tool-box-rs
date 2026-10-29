// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use crate::lib::types::rhea_dclv2_liquidity_type::RHEA_DCLV2_LIQUIDITY_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch a paginated slice of enriched DCL v2 liquidity
/// views owned by `account_id` using raw JSON args.
///
/// `from_index` is the start index (inclusive) and `limit` caps how many
/// views are returned. The enriched fields that are not part of
/// `RHEA_DCLV2_LIQUIDITY_TYPE` are preserved in `extra`.
pub async fn list_liquidity_views(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
    from_index: u64,
    limit: u64,
) -> Result<Vec<RHEA_DCLV2_LIQUIDITY_TYPE>, Error> {
    let out: Vec<RHEA_DCLV2_LIQUIDITY_TYPE> = near
        .view::<Vec<RHEA_DCLV2_LIQUIDITY_TYPE>>(
            dcl_id,
            RHEA_DCLV2_METHODS_CONST.list_liquidity_views,
        )
        .args(json!({ "account_id": account_id, "from_index": from_index, "limit": limit }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
