// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch a single DCL v2 liquidity position by its lpt id
/// using raw JSON args.
///
/// `lpt_id` is the position token id (e.g. `pool_id:left_point:right_point`
/// suffixed form minted by the DCL contract). Returns the raw liquidity
/// object.
pub async fn get_liquidity(near: &Near, dcl_id: &str, lpt_id: &str) -> Result<Value, Error> {
    let liq: Value = near
        .view::<Value>(dcl_id, "get_liquidity")
        .args(json!({ "lpt_id": lpt_id }))
        .await?;
    Ok(liq)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
