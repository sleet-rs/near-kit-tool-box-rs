// use near_kit::*;
use crate::lib::methods::methods_wrap_near::WRAP_NEAR_METHODS_CONST;
use near_kit::{Error, Near, NearToken};
// =================================================
/// Wrap NEAR into wNEAR via the `wrap.near` / `wrap.testnet` contract.
///
/// `amount` is the quantity of NEAR to wrap (attached as deposit).
/// The `near_deposit` method takes no args — just the attached deposit.
pub async fn wrap_near_deposit(
    near: &Near,
    wrap_contract_id: &str,
    amount: NearToken,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let result = near
        .call(wrap_contract_id, WRAP_NEAR_METHODS_CONST.near_deposit)
        .args(serde_json::json!({}))
        .deposit(amount)
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
