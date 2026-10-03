// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: quote an exact-output swap across DCL v2 pools using raw
/// JSON args.
///
/// `pool_ids` is the ordered route, `input_token` / `output_token` are
/// token account ids, and `output_amount` is the desired amount in yocto
/// units of `output_token`. Returns the raw quote object (required input
/// amount, ...).
pub async fn quote_by_output(
    near: &Near,
    dcl_id: &str,
    pool_ids: &[String],
    input_token: &str,
    output_token: &str,
    output_amount: &str,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "quote_by_output")
        .args(json!({
            "pool_ids": pool_ids,
            "input_token": input_token,
            "output_token": output_token,
            "output_amount": output_amount,
        }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
