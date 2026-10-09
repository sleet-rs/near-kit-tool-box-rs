// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use near_kit::{Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Change helper: withdraw excess storage NEAR from the DCL v2 contract
/// using raw JSON args.
///
/// Takes no args. **Attaches exactly 1 yoctoNEAR** (required by the
/// contract) — omitting it panics with "Requires attached deposit of
/// exactly 1 yoctoNEAR".
///
/// Fails if it would drop below the locked storage for live liquidity /
/// order slots — remove those first.
pub async fn storage_withdraw(
    near: &Near,
    dcl_id: &str,
) -> Result<near_kit::rpc::FinalExecutionOutcome, Error> {
    let result = near
        .call(dcl_id, RHEA_DCLV2_METHODS_CONST.storage_withdraw)
        .args(json!({}))
        .gas(Gas::from_tgas(300))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near