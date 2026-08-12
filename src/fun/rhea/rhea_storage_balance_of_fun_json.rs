// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use crate::lib::types::storage_balance_of_response_type::STORAGE_BALANCE_OF_RESPONSE_TYPE;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the NEP-145 storage balance that `account_id`
/// holds on the rhea / ref-finance DEX contract using raw JSON args.
///
/// Returns the standard `{ total, available }` (both yoctoNEAR
/// decimal strings). Use `storage_deposit` on the contract to
/// register storage.
pub async fn storage_balance_of(
    near: &Near,
    rhea_contract_id: &str,
    account_id: &AccountId,
) -> Result<STORAGE_BALANCE_OF_RESPONSE_TYPE, Error> {
    let balance: STORAGE_BALANCE_OF_RESPONSE_TYPE = near
        .view::<STORAGE_BALANCE_OF_RESPONSE_TYPE>(
            rhea_contract_id,
            RHEA_METHODS_CONST.storage_balance_of,
        )
        .args(json!({ "account_id": account_id }))
        .await?;
    Ok(balance)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
