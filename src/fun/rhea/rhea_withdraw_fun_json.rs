// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// Change helper: withdraw `amount` of `token_id` from the signer's
/// rhea / ref-finance DEX LP position to the signer's account using
/// raw JSON args.
///
/// `token_id` is the rhea inner token id for the pool (e.g. `"2794"`
/// for pool id 2794, or the full `<pool_id>:<token_account>` form).
/// `amount` is a decimal string in the token's smallest unit.
///
/// Unregisters the LP position when this is the last share; skips
/// auto-unwrapping wNEAR back into NEAR.
pub async fn withdraw(
    near: &Near,
    rhea_contract_id: &str,
    token_id: &str,
    amount: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let result = near
        .call(rhea_contract_id, RHEA_METHODS_CONST.withdraw)
        .args(json!({
            "amount": amount,
            "token_id": token_id,
            "unregister": false,
            "skip_unwrap_near": false,
        }))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
