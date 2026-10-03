// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: list the still-open DCL v2 limit orders of an account
/// using raw JSON args.
///
/// Returns the raw order objects.
pub async fn list_active_orders(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
) -> Result<Vec<Value>, Error> {
    let out: Vec<Value> = near
        .view::<Vec<Value>>(dcl_id, "list_active_orders")
        .args(json!({ "account_id": account_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
