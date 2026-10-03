// use near_kit::*;
use crate::lib::methods::methods_dclv2::DCLV2_METHODS_CONST;
use near_kit::{Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Change helper: withdraw an internal token balance out of the DCL v2
/// contract to the signer's account using raw JSON args.
///
/// `token_id` is the FT contract. Pass `amount` (`Some`) for a partial
/// withdrawal, or `None` to withdraw the full balance (the call shape is
/// then just `{token_id}`).
///
/// Check `get_user_asset` / `list_user_assets` first. Takes no deposit.
pub async fn withdraw_asset(
    near: &Near,
    dcl_id: &str,
    token_id: &str,
    amount: Option<&str>,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let mut args = json!({ "token_id": token_id });
    if let Some(a) = amount {
        args["amount"] = json!(a);
    }
    let result = near
        .call(dcl_id, DCLV2_METHODS_CONST.withdraw_asset)
        .args(args)
        .gas(Gas::from_tgas(300))
        .deposit(NearToken::from_yoctonear(0))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near