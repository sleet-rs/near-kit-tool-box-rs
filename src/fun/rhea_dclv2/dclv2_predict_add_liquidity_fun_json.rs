// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: simulate adding liquidity to a DCL v2 pool range using
/// raw JSON args.
///
/// `left_point` / `right_point` bound the tick range (`i32`) and
/// `amount_x` / `amount_y` are the candidate deposits in yocto units.
/// Returns the raw prediction (minted liquidity, required amounts, ...).
pub async fn predict_add_liquidity(
    near: &Near,
    dcl_id: &str,
    pool_id: &str,
    left_point: i32,
    right_point: i32,
    amount_x: &str,
    amount_y: &str,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "predict_add_liquidity")
        .args(json!({
            "pool_id": pool_id,
            "left_point": left_point,
            "right_point": right_point,
            "amount_x": amount_x,
            "amount_y": amount_y,
        }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
