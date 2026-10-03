// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch one inner token balance an account holds inside the
/// DCL v2 contract using raw JSON args.
///
/// `token_id` is the underlying FT account id. Returns the raw asset
/// object (yocto string amount, ...).
pub async fn get_user_asset(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
    token_id: &str,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "get_user_asset")
        .args(json!({ "account_id": account_id, "token_id": token_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
