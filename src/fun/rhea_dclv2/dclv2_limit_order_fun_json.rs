// use near_kit::*;
use crate::lib::methods::methods_ft::FT_METHODS_CONST;
use near_kit::{AccountId, Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Change helper: place a DCL v2 limit order via `ft_transfer_call` on the
/// **sell** token contract, using raw JSON args.
///
/// DCL exposes no `place_order` method — orders enter through the sell
/// token's `ft_transfer_call` → DCL `ft_on_transfer`. The `msg` is
/// `{"LimitOrder":{...}}` (a limit order that is *not* filled immediately)
/// or `{"LimitOrderWithSwap":{...}}` (fills what it can right away, then
/// rests the remainder as a limit order).
///
/// Required `msg` fields (all of them — serde rejects a missing one):
/// - `pool_id` — `token_x|token_y|fee`
/// - `buy_token` — the token the order buys (the token you do **not** send)
/// - `point` — the limit price point; must be a multiple of the pool's
///   `point_delta` (fee 100→1, 400→8, 2000→40, 10000→200) and inside the
///   pool's legal point range, else `E202: illegal point`.
/// - `client_id` — opaque string, pass `""`.
///
/// Do **not** pass `referral_id: ""` — the contract parses it as an account
/// id and an empty string fails with
/// `E600: invalid msg: Error("the account ID is invalid")`. Omit the field
/// entirely to leave it unset.
///
/// `amount` is yocto of `sell_token`. Attaches 1 yocto.
///
/// Gas: the swap-and-rest path runs a real swap before parking the
/// remainder, so it needs well over the 50 TGas that the generic
/// `ft_transfer_call` helper uses — this helper attaches 300 TGas.
pub async fn limit_order(
    near: &Near,
    sell_token_id: &str,
    amount: &str,
    dcl_id: &str,
    pool_id: &str,
    buy_token: &str,
    point: i64,
    with_swap: bool,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let variant = if with_swap {
        "LimitOrderWithSwap"
    } else {
        "LimitOrder"
    };
    let msg = serde_json::to_string(&json!({
        variant: {
            "pool_id": pool_id,
            "buy_token": buy_token,
            "point": point,
            "client_id": "",
        }
    }))
    .map_err(|e| Error::Config(format!("limit order msg encode: {e}")))?;
    let receiver: AccountId = dcl_id.parse()?;
    let result = near
        .call(sell_token_id, FT_METHODS_CONST.ft_transfer_call)
        .args(json!({
            "receiver_id": receiver,
            "amount": amount,
            "msg": msg,
        }))
        .gas(Gas::from_tgas(300))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
