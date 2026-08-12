// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch the total number of pools registered on the
/// rhea / ref-finance DEX using raw JSON args.
///
/// Calls `get_number_of_pools` with `{}` and returns the result as a
/// raw `serde_json::Value` (e.g. a JSON number for the pool count).
pub async fn get_number_of_pools(
    near: &Near,
    rhea_contract_id: &str,
) -> Result<Value, Error> {
    let number_of_pools: Value = near
        .view::<Value>(rhea_contract_id, RHEA_METHODS_CONST.get_number_of_pools)
        .args(json!({}))
        .await?;
    Ok(number_of_pools)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
