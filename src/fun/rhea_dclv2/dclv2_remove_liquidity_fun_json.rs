// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use near_kit::{Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Change helper: remove concentrated liquidity from a DCL v2 position
/// using raw JSON args.
///
/// `lpt_id` is the position id (`pool_id#id`, from `list_liquidities` /
/// `get_liquidity`). `amount` is the **liquidity amount** (not a token
/// amount) — read it from the position, or pre-compute with
/// `predict_remove_liquidity`. `min_amount_x` / `min_amount_y` are
/// required fields; `None` is sent as `"0"`.
///
/// Tokens land in the signer's internal balance — pull them out with
/// [`crate::fun::rhea_dclv2::dclv2_withdraw_asset_fun_json::withdraw_asset`].
///
/// Takes **no** attached deposit. Attaching one fails with
/// "Method remove_liquidity doesn't accept deposit".
pub async fn remove_liquidity(
    near: &Near,
    dcl_id: &str,
    lpt_id: &str,
    amount: &str,
    min_amount_x: Option<&str>,
    min_amount_y: Option<&str>,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let result = near
        .call(dcl_id, RHEA_DCLV2_METHODS_CONST.remove_liquidity)
        .args(json!({
            "lpt_id": lpt_id,
            "amount": amount,
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