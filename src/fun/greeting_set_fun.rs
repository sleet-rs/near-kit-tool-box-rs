// use near_kit::*;
use crate::lib::contract_greeting::GREETING_CONTRACT_TRAIT;
use near_kit::{Error, Near};
// =================================================
/// Change helper: set a new greeting on the contract.
///
/// Requires a `Near` client configured with credentials for the signer
/// that is allowed to call `set_greeting` on `contract_id`.
pub async fn set_greeting(
    near: &Near,
    contract_id: &str,
    new_greeting: String,
) -> Result<(), Error> {
    // Use the trait object that the macro implements `Contract` for:
    let greeter = near.contract::<dyn GREETING_CONTRACT_TRAIT>(contract_id);

    // This sends a change method transaction.
    greeter.set_greeting(new_greeting).await?;

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
