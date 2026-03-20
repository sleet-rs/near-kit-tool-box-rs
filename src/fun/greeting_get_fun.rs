// use near_kit::*;
use near_kit::{Error, Near};
use crate::lib::contract_greeting::GREETING_CONTRACT_TRAIT;
// =================================================
/// View helper: fetch the current greeting.
pub async fn get_greeting_fun(near: &Near, contract_id: &str) -> Result<String, Error> {
    // Use the trait object that the macro implements `Contract` for:
    let greeter = near.contract::<dyn GREETING_CONTRACT_TRAIT>(contract_id);
    let greeting = greeter.get_greeting().await?;
    Ok(greeting)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near