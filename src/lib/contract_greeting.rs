// use near_kit::*;
// =================================================
/// Typed interface for a simple greeting contract.
#[near_kit::contract]
pub trait GREETING_CONTRACT_TRAIT {
    fn get_greeting(&self) -> String;
    #[call]
    fn set_greeting(&mut self, new_greeting: String);
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
