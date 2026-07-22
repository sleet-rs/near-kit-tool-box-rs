// =================================================
/// Contract id constants for the `near` (mainnet) / `testnet`
/// (testnet) TLD registrar / linkdrop contracts.
///
/// Both contracts are the same `near-linkdrop` source compiled for
/// each network — only the deploy account differs.
pub const NEAR_CONTRACT_ID_CONST: NEAR_CONTRACT_ID_CONST_TYPE = NEAR_CONTRACT_ID_CONST_TYPE {
    testnet: "testnet",
    mainnet: "near",
};
/// String constants for the TLD registrar / linkdrop contract ids
/// per network.
pub type NEAR_CONTRACT_ID_CONST_TYPE = super::contract_id_const_type::CONTRACT_ID_CONST_TYPE;
// =================================================
/// Returns the TLD registrar / linkdrop contract id for the given
/// network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn near_contractid_fun(network: &str) -> &'static str {
    match network {
        "mainnet" => NEAR_CONTRACT_ID_CONST.mainnet,
        "testnet" => NEAR_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near