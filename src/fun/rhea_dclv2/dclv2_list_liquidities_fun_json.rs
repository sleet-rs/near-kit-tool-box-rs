// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch a paginated slice of DCL v2 liquidity positions
/// owned by `account_id` using raw JSON args.
///
/// `from_index` is the start index (inclusive) and `limit` caps how many
/// positions are returned.
pub async fn list_liquidities(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
    from_index: u64,
    limit: u64,
) -> Result<Vec<Value>, Error> {
    let out: Vec<Value> = near
        .view::<Vec<Value>>(dcl_id, "list_liquidities")
        .args(json!({ "account_id": account_id, "from_index": from_index, "limit": limit }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
