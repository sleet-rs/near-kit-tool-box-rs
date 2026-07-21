// use near_kit::*;
use crate::lib::methods::methods_greeting::GREETING_METHODS_CONST;
use near_kit::{Error, Near};
use serde_json::json;
// =================================================
/// View helper: fetch the current greeting using raw JSON args.
///
/// Mirrors `greeting_set_fun_json::set_greeting` but for the read-only
/// `get_greeting` view call. No signer is required.
pub async fn get_greeting_json(near: &Near, contract_id: &str) -> Result<String, Error> {
    let greeting: String = near
        .view::<String>(contract_id, GREETING_METHODS_CONST.get_greeting)
        .args(json!({}))
        .await?;
    Ok(greeting)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
