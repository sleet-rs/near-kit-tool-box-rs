// =================================================
/// Method name constants for the `near` / `testnet` top-level domain
/// (TLD) registrar contracts.
///
/// The `create_account` method is exposed on both the `near` contract
/// (mainnet) and the `testnet` contract (testnet). Verify with:
///
/// ```sh
/// near contract inspect near       network-config mainnet now
/// near contract inspect testnet    network-config testnet now
/// ```
pub const NEAR_METHODS_CONST: NEAR_METHODS_CONST_TYPE = NEAR_METHODS_CONST_TYPE {
    create_account: "create_account",
};
// =================================================
/// String constants for the TLD registrar contract method names.
pub struct NEAR_METHODS_CONST_TYPE {
    pub create_account: &'static str,
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
