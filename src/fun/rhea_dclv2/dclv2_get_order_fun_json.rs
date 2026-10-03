// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch a single DCL v2 limit order by its order id using
/// raw JSON args.
///
/// Returns the raw order object (owner, pool, point, amounts, ...).
pub async fn get_order(near: &Near, dcl_id: &str, order_id: &str) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "get_order")
        .args(json!({ "order_id": order_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
