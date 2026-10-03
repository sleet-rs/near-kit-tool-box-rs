// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: list every inner token balance an account holds inside
/// the DCL v2 contract using raw JSON args.
///
/// Returns the raw asset map.
pub async fn list_user_assets(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "list_user_assets")
        .args(json!({ "account_id": account_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
