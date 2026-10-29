// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use crate::lib::types::rhea_dclv2_liquidity_type::RHEA_DCLV2_LIQUIDITY_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the enriched view of a single DCL v2 liquidity
/// position by its lpt id using raw JSON args.
///
/// Unlike `get_liquidity`, the view also resolves derived fields (owed
/// fees, current value, ...). The enriched fields that are not part of
/// `RHEA_DCLV2_LIQUIDITY_TYPE` are preserved in `extra`.
pub async fn get_liquidity_view(
    near: &Near,
    dcl_id: &str,
    lpt_id: &str,
) -> Result<RHEA_DCLV2_LIQUIDITY_TYPE, Error> {
    let out: RHEA_DCLV2_LIQUIDITY_TYPE = near
        .view::<RHEA_DCLV2_LIQUIDITY_TYPE>(dcl_id, RHEA_DCLV2_METHODS_CONST.get_liquidity_view)
        .args(json!({ "lpt_id": lpt_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
