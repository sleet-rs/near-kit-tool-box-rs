// use near_kit::*;
use near_kit::{AccountId, Error, Near};
// =================================================
/// Delete the signer's account and transfer any remaining balance to
/// `beneficiary` using the NEAR transaction-builder API.
pub async fn delete_account(
    near: &Near,
    signer_id: &str,
    beneficiary: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let beneficiary_id: AccountId = beneficiary.parse()?;
    let result = near
        .transaction(signer_id)
        .delete_account(beneficiary_id)
        .send()
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
