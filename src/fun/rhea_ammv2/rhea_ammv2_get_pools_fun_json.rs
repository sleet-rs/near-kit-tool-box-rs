// use near_kit::*;
use crate::lib::methods::methods_rhea_ammv2::RHEA_AMMV2_METHODS_CONST;
use crate::lib::types::rhea_ammv2_get_pool_type::RHEA_AMMV2_GET_POOL_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch a paginated slice of pools from the rhea_ammv2 /
/// ref-finance DEX using raw JSON args.
///
/// `from_index` is the start index (inclusive) and `limit` is the
/// maximum number of pools to return.
pub async fn get_pools(
    near: &Near,
    rhea_ammv2_contract_id: &str,
    from_index: u64,
    limit: u64,
) -> Result<Vec<RHEA_AMMV2_GET_POOL_TYPE>, Error> {
    let pools: Vec<RHEA_AMMV2_GET_POOL_TYPE> = near
        .view::<Vec<RHEA_AMMV2_GET_POOL_TYPE>>(rhea_ammv2_contract_id, RHEA_AMMV2_METHODS_CONST.get_pools)
        .args(json!({ "from_index": from_index, "limit": limit }))
        .await?;
    Ok(pools)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
