// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the FT balance of `account_id` using raw JSON
/// args.
///
/// Returns the balance as a decimal string in the token's smallest
/// unit (e.g. yoctoNEAR for wNEAR).
pub async fn ft_balance_of(
    near: &Near,
    ft_contract_id: &str,
    account_id: &str,
) -> Result<String, Error> {
    let account: AccountId = account_id.parse()?;
    let balance: String = near
        .view::<String>(ft_contract_id, FT_METHODS_CONST.ft_balance_of)
        .args(json!({ "account_id": account }))
        .await?;
    Ok(balance)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
