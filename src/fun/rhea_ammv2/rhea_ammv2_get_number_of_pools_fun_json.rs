// use near_kit::*;
use crate::lib::methods::methods_rhea_ammv2::RHEA_AMMV2_METHODS_CONST;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the total number of pools registered on the
/// rhea_ammv2 / ref-finance DEX using raw JSON args.
pub async fn get_number_of_pools(near: &Near, rhea_ammv2_contract_id: &str) -> Result<u64, Error> {
    let number_of_pools: u64 = near
        .view::<u64>(rhea_ammv2_contract_id, RHEA_AMMV2_METHODS_CONST.get_number_of_pools)
        .args(json!({}))
        .await?;
    Ok(number_of_pools)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
