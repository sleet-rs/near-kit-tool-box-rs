// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use near_kit::{Error, IntoNearToken, Near};
use serde_json::json;
// =================================================
/// Register storage on the FT contract for the signer using raw JSON
/// args.
///
/// Attaches the standard storage deposit of 0.00125 NEAR. Takes no
/// args — the contract infers the account from the signer.
pub async fn ft_storage_deposit(
    near: &Near,
    ft_contract_id: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let deposit = "0.00125 NEAR".into_near_token()?;
    let result = near
        .call(ft_contract_id, FT_METHODS_CONST.storage_deposit)
        .args(json!({}))
        .deposit(deposit)
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
