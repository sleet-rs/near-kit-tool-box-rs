// use near_kit::*;
use crate::lib::contract::contract_greeting::GREETING_CONTRACT_TRAIT;
use near_kit::{AccountId, Error, Near};
// =================================================
/// View helper: fetch the current greeting.
pub async fn get_greeting(near: &Near, contract_id: &str) -> Result<String, Error> {
    let contract_id: AccountId = contract_id
        .parse()
        .map_err(|e| Error::Config(format!("invalid account id `{contract_id}`: {e}")))?;
    // Use the trait object that the macro implements `Contract` for:
    let greeter = near.contract::<GREETING_CONTRACT_TRAIT>(contract_id);
    let greeting = greeter.get_greeting().await?;
    Ok(greeting)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
