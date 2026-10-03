// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use near_kit::{AccountId, Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Change helper: fund the signer's DCL v2 internal balance by calling
/// `ft_transfer_call` on the input FT contract using raw JSON args.
///
/// DCL has NO direct `swap` / deposit entrypoint — everything enters via
/// the token contract. `msg` must be the JSON string `"Deposit"`
/// (including the quotes) which credits the internal balance
/// (`get_user_asset` / `list_user_assets`); `add_liquidity` then pulls
/// from that balance. (`Swap` / `LimitOrderWithSwap` msgs swap instead —
/// see `dclv2_swap_ft_transfer_call_fun_json`.)
///
/// `amount` is yocto of `token_id`. Attaches 1 yocto.
pub async fn deposit(
    near: &Near,
    token_id: &str,
    amount: &str,
    dcl_id: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let receiver: AccountId = dcl_id.parse()?;
    let result = near
        .call(token_id, FT_METHODS_CONST.ft_transfer_call)
        .args(json!({
            "receiver_id": receiver,
            "amount": amount,
            "msg": "\"Deposit\"",
        }))
        .gas(Gas::from_tgas(1000))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
