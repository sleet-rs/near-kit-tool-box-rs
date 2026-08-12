// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use crate::lib::types::ref_get_pool_type::REF_GET_POOL_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch a single pool by its pool id from the rhea /
/// ref-finance DEX using raw JSON args.
///
/// `pool_id` is the numeric pool id assigned when the pool was
/// created.
pub async fn get_pool(
    near: &Near,
    rhea_contract_id: &str,
    pool_id: u32,
) -> Result<REF_GET_POOL_TYPE, Error> {
    let pool: REF_GET_POOL_TYPE = near
        .view::<REF_GET_POOL_TYPE>(rhea_contract_id, RHEA_METHODS_CONST.get_pool)
        .args(json!({ "pool_id": pool_id }))
        .await?;
    Ok(pool)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
