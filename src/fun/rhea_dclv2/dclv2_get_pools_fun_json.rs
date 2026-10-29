// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use crate::lib::types::rhea_dclv2_pool_type::RHEA_DCLV2_POOL_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch several DCL v2 pools by their pool ids using raw
/// JSON args.
///
/// Each entry of `pool_ids` is a `<hash>|<token>|<fee>` pool id string.
/// Returns the pool objects in the same order.
pub async fn get_pools(
    near: &Near,
    dcl_id: &str,
    pool_ids: &[String],
) -> Result<Vec<RHEA_DCLV2_POOL_TYPE>, Error> {
    let pools: Vec<RHEA_DCLV2_POOL_TYPE> = near
        .view::<Vec<RHEA_DCLV2_POOL_TYPE>>(dcl_id, RHEA_DCLV2_METHODS_CONST.get_pools)
        .args(json!({ "pool_ids": pool_ids }))
        .await?;
    Ok(pools)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
