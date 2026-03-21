// use near_kit::*;
// use crate::lib::contract_greeting::GREETING_CONTRACT_TRAIT;
use near_kit::{Error, Gas, Near};
use serde_json::json;
// =================================================
/// Change helper: set a new greeting on the contract using raw JSON args.
///
/// Requires a `Near` client configured with credentials for the signer
/// that is allowed to call `set_greeting` on `contract_id`.
pub async fn set_greeting(
    near: &Near,
    contract_id: &str,
    new_greeting: String,
) -> Result<(), Error> {
    near.call(contract_id, "set_greeting")
        .args(json!({ "greeting": new_greeting }))
        .gas(Gas::from_tgas(3))
        .await?;
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
