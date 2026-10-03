// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch the enriched view of a single DCL v2 liquidity
/// position by its lpt id using raw JSON args.
///
/// Unlike `get_liquidity`, the view also resolves derived fields (owed
/// fees, current value, ...). Returns the raw liquidity-view object.
pub async fn get_liquidity_view(
    near: &Near,
    dcl_id: &str,
    lpt_id: &str,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "get_liquidity_view")
        .args(json!({ "lpt_id": lpt_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
