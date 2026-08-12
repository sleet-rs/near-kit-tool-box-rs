// use near_kit::*;
use crate::lib::methods::methods_rhea::RHEA_METHODS_CONST;
use near_kit::{Error, Near};
use serde_json::{Value, json};
// =================================================
/// Change helper: perform a swap (or routed multi-hop swap) on the
/// rhea / ref-finance DEX using raw JSON args.
///
/// `actions_json` is the JSON value for the rhea `actions` array. Each
/// action is an object like
/// `{ "pool_id": 2794, "token_in": "wrap.near", "token_out": "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.near", "min_amount_out": "0" }`
/// plus an optional `amount_in`. Accepting a pre-built JSON value lets
/// callers compose the action list however they like without us
/// pinning a Rust struct to the wire shape.
///
/// `referral_id` is the optional NEAR account that should be credited
/// as the referrer (pass an empty string for none).
///
/// No deposit is attached — the swap routes user tokens already held
/// by the contract under the signer's storage registration.
pub async fn swap(
    near: &Near,
    rhea_contract_id: &str,
    actions_json: Value,
    referral_id: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let result = near
        .call(rhea_contract_id, RHEA_METHODS_CONST.swap)
        .args(json!({
            "actions": actions_json,
            "referral_id": referral_id,
        }))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
