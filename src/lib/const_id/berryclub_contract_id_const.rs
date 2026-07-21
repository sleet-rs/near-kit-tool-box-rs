// =================================================
/// Contract id constants for the berryclub.ek.near pixel game contract.
pub const BERRYCLUB_CONTRACT_ID_CONST: BERRYCLUB_CONTRACT_ID_CONST_TYPE =
    BERRYCLUB_CONTRACT_ID_CONST_TYPE {
        testnet: "berryclub.testnet",
        mainnet: "berryclub.ek.near",
    };
// =================================================
/// String constants for berryclub contract ids per network.
pub struct BERRYCLUB_CONTRACT_ID_CONST_TYPE {
    pub testnet: &'static str,
    pub mainnet: &'static str,
}
// =================================================
/// Returns the berryclub contract id for the given network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn berryclub_contractid_fun(network: &str) -> &'static str {
    match network {
        "mainnet" => BERRYCLUB_CONTRACT_ID_CONST.mainnet,
        "testnet" => BERRYCLUB_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
