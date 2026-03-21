use serde::Serialize;
// =================================================
/// Arguments for `set_greeting` call.
#[derive(Serialize)]
pub struct SET_GREETING_ARGS {
    pub greeting: String,
}
// =================================================
/// Typed interface for a simple greeting contract.
#[near_kit::contract]
pub trait GREETING_CONTRACT_TRAIT {
    fn get_greeting(&self) -> String;
    #[call]
    fn set_greeting(&mut self, args: SET_GREETING_ARGS);
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
