// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use near_kit::{AccountId, Error, Near, NearToken};
use serde_json::json;
// =================================================
/// Transfer FT to `receiver_id` using raw JSON args.
///
/// `amount` is the decimal string in the token's smallest unit
/// (e.g. `"1000000000000000000000000"` = 1 wNEAR). The contract
/// requires exactly 1 yoctoNEAR attached as deposit for security.
pub async fn ft_transfer(
    near: &Near,
    ft_contract_id: &str,
    receiver_id: &str,
    amount: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let receiver: AccountId = receiver_id.parse()?;
    let result = near
        .call(ft_contract_id, FT_METHODS_CONST.ft_transfer)
        .args(json!({
            "receiver_id": receiver,
            "amount": amount,
        }))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
