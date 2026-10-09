// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use near_kit::{AccountId, Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Change helper: swap on DCL v2 by calling `ft_transfer_call` on the
/// INPUT token contract using raw JSON args.
///
/// DCL exposes no `swap` method (`near contract inspect` proves it) —
/// swaps enter through the token's `ft_transfer_call` → DCL
/// `ft_on_transfer`. `msg` is `{"Swap":{"pool_ids":[...],
/// "output_token":..., "min_output_amount":...}}`. Get the expected
/// output first via the `quote` view helper and apply slippage.
///
/// Attaches 1 yocto. The output token must have storage registered for
/// the signer or the swap fails.
pub async fn swap(
    near: &Near,
    token_in_id: &str,
    amount_in: &str,
    dcl_id: &str,
    pool_ids: &[String],
    output_token: &str,
    min_output_amount: &str,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let msg = serde_json::to_string(&json!({
        "Swap": {
            "pool_ids": pool_ids,
            "output_token": output_token,
            "min_output_amount": min_output_amount,
        }
    }))
    .map_err(|e| Error::Config(format!("swap msg encode: {e}")))?;
    let receiver: AccountId = dcl_id.parse()?;
    let result = near
        .call(token_in_id, FT_METHODS_CONST.ft_transfer_call)
        .args(json!({
            "receiver_id": receiver,
            "amount": amount_in,
            "msg": msg,
        }))
        .gas(Gas::from_tgas(180))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
