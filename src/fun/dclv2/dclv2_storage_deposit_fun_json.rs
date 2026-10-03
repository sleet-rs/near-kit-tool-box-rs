// use near_kit::*;
use crate::lib::methods::methods_dclv2::DCLV2_METHODS_CONST;
use near_kit::{Error, IntoNearToken, Near};
use serde_json::json;
// =================================================
/// Change helper: register storage on the DCL v2 contract for the signer
/// using raw JSON args.
///
/// DCL charges per liquidity / order slot (`slot_price` 0.01 NEAR, 40 max
/// on testnet at time of writing — see `get_user_storage_detail`). A bare
/// minimum `storage_balance_bounds` deposit is NOT enough in practice;
/// 0.5 NEAR worked on `dclv2.ref-dev.testnet`.
pub async fn storage_deposit(
    near: &Near,
    dcl_id: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let deposit = "0.5 NEAR".into_near_token()?;
    let result = near
        .call(dcl_id, DCLV2_METHODS_CONST.storage_deposit)
        .args(json!({ "registration_only": true }))
        .deposit(deposit)
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
