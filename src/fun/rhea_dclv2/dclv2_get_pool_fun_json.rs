// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use crate::lib::types::rhea_dclv2_pool_type::RHEA_DCLV2_POOL_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch a single DCL v2 pool by its pool id using raw
/// JSON args.
///
/// `pool_id` is the composite pool id used by the contract, in the form
/// `<hash>|<token>|<fee>`.
pub async fn get_pool(
    near: &Near,
    dcl_id: &str,
    pool_id: &str,
) -> Result<RHEA_DCLV2_POOL_TYPE, Error> {
    let out: RHEA_DCLV2_POOL_TYPE = near
        .view::<RHEA_DCLV2_POOL_TYPE>(dcl_id, RHEA_DCLV2_METHODS_CONST.get_pool)
        .args(json!({ "pool_id": pool_id }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
