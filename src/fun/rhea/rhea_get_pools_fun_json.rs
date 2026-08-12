// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch a paginated slice of pools from the rhea /
/// ref-finance DEX using raw JSON args.
///
/// `from_index` is the start index (inclusive) and `limit` is the
/// maximum number of pools to return. Returns the pools as a raw
/// `serde_json::Value` list.
pub async fn get_pools(
    near: &Near,
    rhea_contract_id: &str,
    from_index: u64,
    limit: u64,
) -> Result<Value, Error> {
    let pools: Value = near
        .view::<Value>(rhea_contract_id, RHEA_METHODS_CONST.get_pools)
        .args(json!({ "from_index": from_index, "limit": limit }))
        .await?;
    Ok(pools)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
