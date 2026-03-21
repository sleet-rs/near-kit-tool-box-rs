// use near_kit::*;
use crate::lib::contract_greeting::{GREETING_CONTRACT_TRAIT, SET_GREETING_ARGS};
use near_kit::{Error, Near};
// =================================================
/// Change helper: set a new greeting on the contract using typed contract interface.
///
/// Requires a `Near` client configured with credentials for the signer
/// that is allowed to call `set_greeting` on `contract_id`.
pub async fn set_greeting_typed(
    near: &Near,
    contract_id: &str,
    new_greeting: String,
) -> Result<(), Error> {
    // Use the typed contract interface - compile-time type checking
    let greeter = near.contract::<dyn GREETING_CONTRACT_TRAIT>(contract_id);
    greeter
        .set_greeting(SET_GREETING_ARGS { greeting: new_greeting })
        .await?;
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
