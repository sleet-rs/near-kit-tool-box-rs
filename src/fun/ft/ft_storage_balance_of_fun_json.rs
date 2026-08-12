// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use crate::lib::types::storage_balance_of_response_type::STORAGE_BALANCE_OF_RESPONSE_TYPE;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the NEP-145 storage balance that `account_id`
/// holds on an FT contract (e.g. `wrap.testnet`, `usdt.tether-token.near`)
/// using raw JSON args.
///
/// Returns the standard `{ total, available }` (both yoctoNEAR
/// decimal strings), or contract-specific shapes per FT contract.
pub async fn ft_storage_balance_of(
    near: &Near,
    ft_contract_id: &str,
    account_id: &str,
) -> Result<STORAGE_BALANCE_OF_RESPONSE_TYPE, Error> {
    let account: AccountId = account_id.parse()?;
    let balance: STORAGE_BALANCE_OF_RESPONSE_TYPE = near
        .view::<STORAGE_BALANCE_OF_RESPONSE_TYPE>(ft_contract_id, FT_METHODS_CONST.storage_balance_of)
        .args(json!({ "account_id": account }))
        .await?;
    Ok(balance)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
