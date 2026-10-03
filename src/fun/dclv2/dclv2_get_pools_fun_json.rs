// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch several DCL v2 pools by their pool ids using raw
/// JSON args.
///
/// Each entry of `pool_ids` is a `token_x|token_y|fee` string. Returns
/// the raw pool objects in the same order.
pub async fn get_pools(
    near: &Near,
    dcl_id: &str,
    pool_ids: &[String],
) -> Result<Vec<Value>, Error> {
    let pools: Vec<Value> = near
        .view::<Vec<Value>>(dcl_id, "get_pools")
        .args(json!({ "pool_ids": pool_ids }))
        .await?;
    Ok(pools)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
