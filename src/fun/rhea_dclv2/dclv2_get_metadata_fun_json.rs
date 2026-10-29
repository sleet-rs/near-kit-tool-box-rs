// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use crate::lib::types::rhea_dclv2_metadata_type::RHEA_DCLV2_METADATA_TYPE;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the DCL v2 contract metadata using raw JSON args.
///
/// The contract answers with its version, owner, farming contract id,
/// wNEAR id, state, operators, protocol fee rate, pool count, and user
/// count. Unknown fields are preserved in `extra`.
pub async fn get_metadata(near: &Near, dcl_id: &str) -> Result<RHEA_DCLV2_METADATA_TYPE, Error> {
    let out: RHEA_DCLV2_METADATA_TYPE = near
        .view::<RHEA_DCLV2_METADATA_TYPE>(dcl_id, RHEA_DCLV2_METHODS_CONST.get_metadata)
        .args(json!({}))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
