// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch the storage-deposit detail the DCL v2 contract
/// tracks for one user using raw JSON args.
///
/// Returns the raw detail object (paid deposit, used storage, ...).
pub async fn get_user_storage_detail(
    near: &Near,
    dcl_id: &str,
    user_id: &str,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "get_user_storage_detail")
        .args(json!({ "user_id": user_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
