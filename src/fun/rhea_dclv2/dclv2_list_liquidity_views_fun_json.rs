// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch a paginated slice of enriched DCL v2 liquidity
/// views owned by `account_id` using raw JSON args.
///
/// `from_index` is the start index (inclusive) and `limit` caps how many
/// views are returned.
pub async fn list_liquidity_views(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
    from_index: u64,
    limit: u64,
) -> Result<Vec<Value>, Error> {
    let out: Vec<Value> = near
        .view::<Vec<Value>>(dcl_id, "list_liquidity_views")
        .args(json!({ "account_id": account_id, "from_index": from_index, "limit": limit }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
