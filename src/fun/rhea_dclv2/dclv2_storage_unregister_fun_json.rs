// use near_kit::*;
use crate::lib::methods::methods_rhea_dclv2::RHEA_DCLV2_METHODS_CONST;
use near_kit::{Error, Gas, Near, NearToken};
use serde_json::json;
// =================================================
/// Change helper: unregister the signer's storage on the DCL v2 contract
/// using raw JSON args.
///
/// Takes no args. **Attaches exactly 1 yoctoNEAR** (required by the
/// contract). Returns the locked NEAR to the signer. Only works once no
/// liquidity slots / order slots are in use.
pub async fn storage_unregister(
    near: &Near,
    dcl_id: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let result = near
        .call(dcl_id, RHEA_DCLV2_METHODS_CONST.storage_unregister)
        .args(json!({}))
        .gas(Gas::from_tgas(300))
        .deposit(NearToken::from_yoctonear(1))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near