// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: simulate removing liquidity from a DCL v2 position using
/// raw JSON args.
///
/// `lpt_id` is the position token id and `liquidity` is how much of its
/// liquidity to burn (yocto string). Returns the raw prediction
/// (returned token amounts, ...).
pub async fn predict_remove_liquidity(
    near: &Near,
    dcl_id: &str,
    lpt_id: &str,
    liquidity: &str,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "predict_remove_liquidity")
        .args(json!({ "lpt_id": lpt_id, "liquidity": liquidity }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
