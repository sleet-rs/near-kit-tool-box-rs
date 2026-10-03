// use near_kit::*;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch the market-depth snapshot around the current price
/// of a DCL v2 pool using raw JSON args.
///
/// `depth` is how many points on each side of the current tick are
/// aggregated. Returns the raw depth object.
pub async fn get_marketdepth(
    near: &Near,
    dcl_id: &str,
    pool_id: &str,
    depth: u64,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, "get_marketdepth")
        .args(json!({ "pool_id": pool_id, "depth": depth }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
