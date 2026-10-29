// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use crate::lib::types::rhea_dclv2_liquidity_type::RHEA_DCLV2_LIQUIDITY_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch a single DCL v2 liquidity position by its lpt id
/// using raw JSON args.
///
/// `lpt_id` is the position token id, the pool id followed by the
/// owner's liquidity index (for example
/// `nearvember-337.meme-cooking.near|wrap.near|10000#17677`).
pub async fn get_liquidity(
    near: &Near,
    dcl_id: &str,
    lpt_id: &str,
) -> Result<RHEA_DCLV2_LIQUIDITY_TYPE, Error> {
    let liq: RHEA_DCLV2_LIQUIDITY_TYPE = near
        .view::<RHEA_DCLV2_LIQUIDITY_TYPE>(dcl_id, RHEA_DCLV2_METHODS_CONST.get_liquidity)
        .args(json!({ "lpt_id": lpt_id }))
        .await?;
    Ok(liq)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
