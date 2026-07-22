// =================================================
/// Contract id constants for the wrap.near wNEAR contract.
pub const WRAP_NEAR_CONTRACT_ID_CONST: WRAP_NEAR_CONTRACT_ID_CONST_TYPE =
    WRAP_NEAR_CONTRACT_ID_CONST_TYPE {
        testnet: "wrap.testnet",
        mainnet: "wrap.near",
    };
// =================================================
/// String constants for wrap.near contract ids per network.
pub type WRAP_NEAR_CONTRACT_ID_CONST_TYPE = super::contract_id_const_type::CONTRACT_ID_CONST_TYPE;
// =================================================
/// Returns the wrap.near contract id for the given network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn wrap_near_contractid_fun(network: &str) -> &'static str {
    match network {
        "mainnet" => WRAP_NEAR_CONTRACT_ID_CONST.mainnet,
        "testnet" => WRAP_NEAR_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
