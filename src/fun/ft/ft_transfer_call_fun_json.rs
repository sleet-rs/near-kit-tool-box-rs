// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use near_kit::{AccountId, Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Transfer FT to `receiver_id` and call `ft_transfer_call` on it with
/// `msg`, using raw JSON args.
///
/// `amount` is the decimal string in the token's smallest unit. The
/// contract requires exactly 1 yoctoNEAR attached as deposit.
pub async fn ft_transfer_call(
    near: &Near,
    ft_contract_id: &str,
    receiver_id: &str,
    amount: &str,
    msg: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let receiver: AccountId = receiver_id.parse()?;
    let result = near
        .call(ft_contract_id, FT_METHODS_CONST.ft_transfer_call)
        .args(json!({
            "receiver_id": receiver,
            "amount": amount,
            "msg": msg,
        }))
        .gas(Gas::from_tgas(50))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
