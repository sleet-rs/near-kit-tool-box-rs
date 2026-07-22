// =================================================
/// Method name constants for the `near` (mainnet) and `testnet`
/// (testnet) top-level-domain registrar / linkdrop contracts.
///
/// Both contracts are the same `near-linkdrop` source compiled for
/// each network — only the deploy account differs.
///
/// Verify with:
///
/// ```sh
/// near contract inspect near    network-config mainnet now
/// near contract inspect testnet network-config testnet now
/// ```
pub const NEAR_METHODS_CONST: NEAR_METHODS_CONST_TYPE = NEAR_METHODS_CONST_TYPE {
    claim: "claim",
    contract_source_metadata: "contract_source_metadata",
    create_account: "create_account",
    create_account_advanced: "create_account_advanced",
    create_account_and_claim: "create_account_and_claim",
    get_key_balance: "get_key_balance",
    get_key_information: "get_key_information",
    new: "new",
    on_account_created: "on_account_created",
    on_account_created_and_claimed: "on_account_created_and_claimed",
    send: "send",
};
// =================================================
/// String constants for the TLD registrar / linkdrop contract method
/// names.
pub struct NEAR_METHODS_CONST_TYPE {
    /// Claim tokens for an account attached to the public key that signed the tx. (private)
    pub claim: &'static str,
    /// NEP-330 contract source metadata. (view; mainnet only)
    pub contract_source_metadata: &'static str,
    /// Create a sub-account and deposit the attached funds. (payable)
    pub create_account: &'static str,
    /// Create a sub-account with extra options and deposit the attached funds. (payable)
    pub create_account_advanced: &'static str,
    /// Create a sub-account and claim tokens to it. (private)
    pub create_account_and_claim: &'static str,
    /// View: balance associated with a given public key.
    pub get_key_balance: &'static str,
    /// View: linkdrop key information for a given public key.
    pub get_key_information: &'static str,
    /// Contract initializer.
    pub new: &'static str,
    /// Callback after `create_account` / `create_account_advanced`. (private)
    pub on_account_created: &'static str,
    /// Callback after `create_account_and_claim`. (private)
    pub on_account_created_and_claimed: &'static str,
    /// Allow the given public key to claim an attached balance.
    pub send: &'static str,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near