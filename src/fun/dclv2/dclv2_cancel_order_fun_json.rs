// use near_kit::*;
use crate::lib::methods::methods_dclv2::DCLV2_METHODS_CONST;
use near_kit::{Error, Gas, Near};
use serde_json::json;
// =================================================
/// Change helper: cancel a DCL v2 limit order using raw JSON args.
///
/// `order_id` identifies the order. Pass an `amount` to partially cancel;
/// omit it (`None`) for a full cancel. Pass `"0"` to claim the filled
/// (`unclaimed_amount`) without cancelling the remainder.
pub async fn cancel_order(
    near: &Near,
    dcl_id: &str,
    order_id: &str,
    amount: Option<&str>,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let mut args = json!({ "order_id": order_id });
    if let Some(a) = amount {
        args["amount"] = json!(a);
    }
    let result = near
        .call(dcl_id, DCLV2_METHODS_CONST.cancel_order)
        .args(args)
        .gas(Gas::from_tgas(300))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
