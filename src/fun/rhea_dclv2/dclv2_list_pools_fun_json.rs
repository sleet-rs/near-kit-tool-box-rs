// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use crate::lib::types::rhea_dclv2_pool_type::RHEA_DCLV2_POOL_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch a paginated slice of DCL v2 pools using raw JSON
/// args.
///
/// `from_index` is the start index (inclusive) and `limit` caps how many
/// pool objects are returned.
pub async fn list_pools(
    near: &Near,
    dcl_id: &str,
    from_index: u64,
    limit: u64,
) -> Result<Vec<RHEA_DCLV2_POOL_TYPE>, Error> {
    let pools: Vec<RHEA_DCLV2_POOL_TYPE> = near
        .view::<Vec<RHEA_DCLV2_POOL_TYPE>>(dcl_id, RHEA_DCLV2_METHODS_CONST.list_pools)
        .args(json!({ "from_index": from_index, "limit": limit }))
        .await?;
    Ok(pools)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
