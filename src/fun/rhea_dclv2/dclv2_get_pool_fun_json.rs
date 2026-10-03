// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch a single DCL v2 concentrated-liquidity pool by
/// its pool id using raw JSON args.
///
/// `pool_id` is the `token_x|token_y|fee` string (tokens sorted, e.g.
/// `wrap.near|usdc.near|2000`). Returns the raw pool object.
pub async fn get_pool(near: &Near, dcl_id: &str, pool_id: &str) -> Result<Value, Error> {
    let pool: Value = near
        .view::<Value>(dcl_id, "get_pool")
        .args(json!({ "pool_id": pool_id }))
        .await?;
    Ok(pool)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
