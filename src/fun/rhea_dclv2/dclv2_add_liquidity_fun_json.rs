// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use near_kit::{Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Change helper: add concentrated liquidity to a DCL v2 pool range using
/// raw JSON args.
///
/// Pulls from the signer's internal balances — fund first with
/// [`crate::fun::rhea_dclv2::dclv2_deposit_fun_json::deposit`] (`ft_transfer_call`
/// with msg `"Deposit"`), then verify via `get_user_asset`.
///
/// # args
/// - `pool_id` — `token_x|token_y|fee`; tokens must be sorted.
/// - `left_point` / `right_point` — tick bounds, **must be multiples of the
///   pool's `point_delta`** (fee 100→1, 400→8, 2000→40, 10000→200).
///   Unaligned points fail with `E200: invalid endpoint`.
/// - `amount_x` / `amount_y` — yocto deposits. For a single-sided range pass
///   `"0"` for the side you don't use; a range fully above `current_point`
///   only consumes `amount_x`, fully below only `amount_y`. Use
///   `predict_add_liquidity` first to get the exact `need_x` / `need_y`.
/// - `min_amount_x` / `min_amount_y` — **required** fields (slippage guards);
///   `None` is sent as `"0"`.
///
/// Requires DCL storage registered (0.5 NEAR covers the first liquidity
/// slot). Takes **no** attached deposit — attaching one fails with
/// "Method add_liquidity doesn't accept deposit".
pub async fn add_liquidity(
    near: &Near,
    dcl_id: &str,
    pool_id: &str,
    left_point: i32,
    right_point: i32,
    amount_x: &str,
    amount_y: &str,
    min_amount_x: Option<&str>,
    min_amount_y: Option<&str>,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(dcl_id, RHEA_DCLV2_METHODS_CONST.add_liquidity)
        .args(json!({
            "pool_id": pool_id,
            "left_point": left_point,
            "right_point": right_point,
            "amount_x": amount_x,
            "amount_y": amount_y,
            "min_amount_x": min_amount_x.unwrap_or("0"),
            "min_amount_y": min_amount_y.unwrap_or("0"),
        }))
        .gas(Gas::from_tgas(300))
        .deposit(NearToken::from_yoctonear(0))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near