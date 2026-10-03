// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch a paginated slice of DCL v2 pools using raw JSON
/// args.
///
/// `from_index` is the start index (inclusive) and `limit` caps how many
/// pool objects are returned.
pub async fn list_pools(
    near: &Near,
    dcl_id: &str,
    from_index: u64,
    limit: u64,
) -> Result<Vec<Value>, Error> {
    let pools: Vec<Value> = near
        .view::<Vec<Value>>(dcl_id, "list_pools")
        .args(json!({ "from_index": from_index, "limit": limit }))
        .await?;
    Ok(pools)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
