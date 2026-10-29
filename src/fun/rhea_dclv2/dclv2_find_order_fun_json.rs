// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: find the DCL v2 limit order an account holds on one pool
/// point using raw JSON args.
///
/// `point` is the tick index (`i32`) the order rests on. Returns the raw
/// order object.
pub async fn find_order(
    near: &Near,
    dcl_id: &str,
    account_id: &str,
    pool_id: &str,
    point: i32,
) -> Result<Value, Error> {
    let out: Value = near
        .view::<Value>(dcl_id, RHEA_DCLV2_METHODS_CONST.find_order)
        .args(json!({ "account_id": account_id, "pool_id": pool_id, "point": point }))
        .await?;
    Ok(out)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
