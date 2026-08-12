// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use near_kit::{Error, IntoNearToken, Near};
use serde_json::json;
// =================================================
/// Change helper: register storage on the rhea / ref-finance DEX for
/// the signer using raw JSON args.
///
/// Attaches the standard NEP-145 storage deposit of 0.00125 NEAR.
/// Takes no args — the contract infers the account from the signer.
pub async fn storage_deposit(
    near: &Near,
    rhea_contract_id: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let deposit = "0.00125 NEAR".into_near_token()?;
    let result = near
        .call(rhea_contract_id, RHEA_METHODS_CONST.storage_deposit)
        .args(json!({}))
        .deposit(deposit)
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
