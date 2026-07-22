use near_kit::types::AccountId;
use serde::{Deserialize, Serialize};
// =================================================
/// Arguments for `get_key_balance`.
#[derive(Serialize)]
pub struct NEAR_GET_KEY_BALANCE_ARGS {
    pub key: String,
}
// =================================================
/// Arguments for `get_key_information`.
#[derive(Serialize)]
pub struct NEAR_GET_KEY_INFORMATION_ARGS {
    pub key: String,
}
// =================================================
/// KeyInfo returned by `get_key_information`.
#[derive(Debug, Clone, Deserialize)]
pub struct NEAR_KEY_INFO {
    pub account_id: AccountId,
    pub public_key: String,
    pub allowance: String,
    pub method_names: String,
}
// =================================================
/// Arguments for `create_account`.
#[derive(Serialize)]
pub struct NEAR_CREATE_ACCOUNT_ARGS {
    pub new_account_id: AccountId,
    pub new_public_key: String,
}
// =================================================
/// Arguments for `create_account_advanced`.
#[derive(Serialize)]
pub struct NEAR_CREATE_ACCOUNT_ADVANCED_ARGS {
    pub new_account_id: AccountId,
    pub options: NEAR_CREATE_ACCOUNT_OPTIONS,
}
// =================================================
/// Options bag for `create_account_advanced`.
#[derive(Serialize)]
pub struct NEAR_CREATE_ACCOUNT_OPTIONS {
    pub full_access_keys: Option<Vec<String>>,
    pub function_call_keys: Option<Vec<NEAR_FUNCTION_CALL_KEY>>,
    pub contract_bytes: Option<Vec<u8>>,
}
// =================================================
/// Function-call permission key entry for
/// `create_account_advanced`.
#[derive(Serialize)]
pub struct NEAR_FUNCTION_CALL_KEY {
    pub public_key: String,
    pub allowance: String,
    pub receiver_id: AccountId,
    pub method_names: String,
}
// =================================================
/// Arguments for `send`.
#[derive(Serialize)]
pub struct NEAR_SEND_ARGS {
    pub public_key: String,
}
// =================================================
/// Typed interface for the `near` / `testnet` TLD registrar /
/// linkdrop contract (same `near-linkdrop` source compiled for each
/// network).
#[near_kit::contract]
pub trait NEAR_TLD_CONTRACT_TRAIT {
    /// NEP-330 contract source metadata. (view; mainnet only)
    fn contract_source_metadata(&self);
    /// View: balance associated with a given public key.
    fn get_key_balance(&self, args: NEAR_GET_KEY_BALANCE_ARGS) -> String;
    /// View: linkdrop key information for a given public key.
    fn get_key_information(&self, args: NEAR_GET_KEY_INFORMATION_ARGS) -> NEAR_KEY_INFO;

    /// Create a sub-account and deposit the attached funds. (payable)
    #[call(payable)]
    fn create_account(&mut self, args: NEAR_CREATE_ACCOUNT_ARGS);
    /// Create a sub-account with extra options and deposit the
    /// attached funds. (payable)
    #[call(payable)]
    fn create_account_advanced(&mut self, args: NEAR_CREATE_ACCOUNT_ADVANCED_ARGS);
    /// Allow the given public key to claim an attached balance.
    /// (payable)
    #[call(payable)]
    fn send(&mut self, args: NEAR_SEND_ARGS);
}
// =================================================
// =================================================
// copyright 2026 by sleet.near