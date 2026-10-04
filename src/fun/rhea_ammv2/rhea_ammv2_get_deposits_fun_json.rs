// use near_kit::*;
use crate::lib::methods::methods_rhea_ammv2::RHEA_AMMV2_METHODS_CONST;
use crate::lib::types::rhea_ammv2_get_deposits_type::RHEA_AMMV2_GET_DEPOSITS_TYPE;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch every LP position (per pool) that `account_id`
/// holds on the rhea_ammv2 / ref-finance DEX using raw JSON args.
///
/// Returns a map keyed by `<pool_id>:<token_id>` whose value is the
/// user's LP share balance as a decimal string.
pub async fn get_deposits(
    near: &Near,
    rhea_ammv2_contract_id: &str,
    account_id: &AccountId,
) -> Result<RHEA_AMMV2_GET_DEPOSITS_TYPE, Error> {
    let deposits: RHEA_AMMV2_GET_DEPOSITS_TYPE = near
        .view::<RHEA_AMMV2_GET_DEPOSITS_TYPE>(rhea_ammv2_contract_id, RHEA_AMMV2_METHODS_CONST.get_deposits)
        .args(json!({ "account_id": account_id }))
        .await?;
    Ok(deposits)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
