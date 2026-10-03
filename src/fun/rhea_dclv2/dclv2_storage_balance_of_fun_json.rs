// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch the NEP-145 storage balance of an account on the
/// DCL v2 contract using raw JSON args.
///
/// Returns null when the account has no storage registered, otherwise
/// the raw `{ total, available }` object.
pub async fn storage_balance_of(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "storage_balance_of")
        .args(json!({ "account_id": account_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
