// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use near_kit::{AccountId, Error, Near};
use serde_json::{Value, json};
// =================================================
/// View helper: fetch the NEP-145 storage balance that `account_id`
/// holds on the rhea / ref-finance DEX contract using raw JSON args.
///
/// Returns the storage balance as a raw `serde_json::Value` (or
/// `null` when the account has no storage registration). Use
/// `storage_deposit` on the contract to register storage.
pub async fn storage_balance_of(
    near: &Near,
    rhea_contract_id: &str,
    account_id: &str,
) -> Result<Value, Error> {
    let account: AccountId = account_id.parse()?;
    let balance: Value = near
        .view::<Value>(rhea_contract_id, RHEA_METHODS_CONST.storage_balance_of)
        .args(json!({ "account_id": account }))
        .await?;
    Ok(balance)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
