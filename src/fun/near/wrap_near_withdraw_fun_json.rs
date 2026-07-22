// use near_kit::*;
use crate::lib::methods::methods_wrap_near::WRAP_NEAR_METHODS_CONST;
use near_kit::{Error, Near, NearToken};
use serde_json::json;
// =================================================
/// Unwrap wNEAR back into NEAR via the `wrap.near` / `wrap.testnet`
/// contract.
///
/// `amount` is the quantity of wNEAR to unwrap, expressed in yoctoNEAR
/// (e.g. `"100000000000000000000000"` = 0.1 wNEAR). The contract requires
/// exactly 1 yoctoNEAR attached as deposit for security.
pub async fn wrap_near_withdraw(
    near: &Near,
    wrap_contract_id: &str,
    amount: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let result = near
        .call(wrap_contract_id, WRAP_NEAR_METHODS_CONST.near_withdraw)
        .args(json!({ "amount": amount }))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
