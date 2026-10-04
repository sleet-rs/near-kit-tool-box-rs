// use near_kit::*;
use crate::lib::methods::methods_rhea_ammv2::RHEA_AMMV2_METHODS_CONST;
use crate::lib::types::rhea_ammv2_get_pool_type::RHEA_AMMV2_GET_POOL_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch a single pool by its pool id from the rhea_ammv2 /
/// ref-finance DEX using raw JSON args.
///
/// `pool_id` is the numeric pool id assigned when the pool was
/// created.
pub async fn get_pool(
    near: &Near,
    rhea_ammv2_contract_id: &str,
    pool_id: u32,
) -> Result<RHEA_AMMV2_GET_POOL_TYPE, Error> {
    let pool: RHEA_AMMV2_GET_POOL_TYPE = near
        .view::<RHEA_AMMV2_GET_POOL_TYPE>(rhea_ammv2_contract_id, RHEA_AMMV2_METHODS_CONST.get_pool)
        .args(json!({ "pool_id": pool_id }))
        .await?;
    Ok(pool)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
