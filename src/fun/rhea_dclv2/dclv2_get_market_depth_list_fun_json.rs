// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch market-depth snapshots for several DCL v2 pools at
/// once using raw JSON args.
///
/// `pool_id_list` holds the pool ids and `depth` (when `Some`) sets how
/// many points on each side of each current tick are aggregated.
/// Returns the raw depth objects in the same order.
pub async fn get_market_depth_list(
    near: &Near,
    dcl_id: &str,
    pool_id_list: &[String],
    depth: Option<u64>,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "get_market_depth_list")
        .args(json!({ "pool_id_list": pool_id_list, "depth": depth }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
