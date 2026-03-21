// use near_kit::*;
// use crate::lib::contract_greeting::GREETING_CONTRACT_TRAIT;
use near_kit::{Error, Near, Gas};
use serde_json::json;
// =================================================
/// Change helper: set a new greeting on the contract.
///
/// Requires a `Near` client configured with credentials for the signer
/// that is allowed to call `set_greeting` on `contract_id`.
/// Change helper: set a new greeting on the contract using raw JSON args.
pub async fn set_greeting(
    near: &Near,
    contract_id: &str,
    new_greeting: String,
) -> Result<(), Error> {
    // This will send JSON: { "greeting": "<new_greeting>" }
    // which NEAR RPC then base64-encodes on the wire.
    near.call(contract_id, "set_greeting")
        .args(json!({ "greeting": new_greeting }))
        .gas(Gas::from_tgas(3))
        .await?;

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
