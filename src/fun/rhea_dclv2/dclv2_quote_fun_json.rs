// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: quote an exact-input swap across DCL v2 pools using raw
/// JSON args.
///
/// `pool_ids` is the ordered route, `input_token` / `output_token` are
/// token account ids, `input_amount` is in yocto units of `input_token`,
/// and `tag` is an optional opaque label echoed back in the response.
/// Returns the raw quote object (output amount, tag, ...).
pub async fn quote(
    near: &Near,
    dcl_id: &str,
    pool_ids: &[String],
    input_token: &str,
    output_token: &str,
    input_amount: &str,
    tag: Option<&str>,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "quote")
        .args(json!({
            "pool_ids": pool_ids,
            "input_token": input_token,
            "output_token": output_token,
            "input_amount": input_amount,
            "tag": tag,
        }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
