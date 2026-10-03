// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch the per-point liquidity distribution of a DCL v2
/// pool over `[left_point, right_point]` using raw JSON args.
///
/// Both points are tick indexes (`i32`); `left_point` must be below
/// `right_point`. Returns the raw range object.
pub async fn get_liquidity_range(
    near: &Near,
    dcl_id: &str,
    pool_id: &str,
    left_point: i32,
    right_point: i32,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "get_liquidity_range")
        .args(json!({ "pool_id": pool_id, "left_point": left_point, "right_point": right_point }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
