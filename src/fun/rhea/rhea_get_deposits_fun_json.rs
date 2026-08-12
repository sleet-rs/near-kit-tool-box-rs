// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use near_kit::{AccountId, Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch every LP position (per pool) that `account_id`
/// holds on the rhea / ref-finance DEX using raw JSON args.
///
/// Returns the per-pool deposits as a raw `serde_json::Value` (a list
/// of `{ pool_id, token_id, shares }` entries when the account has
/// positions).
pub async fn get_deposits(
    near: &Near,
    rhea_contract_id: &str,
    account_id: &str,
) -> Result<Value, Error> {
    let account: AccountId = account_id.parse()?;
    let deposits: Value = near
        .view::<Value>(rhea_contract_id, RHEA_METHODS_CONST.get_deposits)
        .args(json!({ "account_id": account }))
        .await?;
    Ok(deposits)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
