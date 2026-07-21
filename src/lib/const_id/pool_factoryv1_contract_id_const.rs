// =================================================
/// Contract id constants for the `poolv1.near` staking-pool factory contract.
pub const POOL_FACTORYV1_CONTRACT_ID_CONST: POOL_FACTORYV1_CONTRACT_ID_CONST_TYPE =
    POOL_FACTORYV1_CONTRACT_ID_CONST_TYPE {
        testnet: "poolv1.testnet",
        mainnet: "poolv1.near",
    };
// =================================================
/// String constants for poolv1.near factory contract ids per network.
pub struct POOL_FACTORYV1_CONTRACT_ID_CONST_TYPE {
    pub testnet: &'static str,
    pub mainnet: &'static str,
}
// =================================================
/// Returns the poolv1.near factory contract id for the given network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn pool_factoryv1_contractid_fun(network: &str) -> &'static str {
    match network {
        "mainnet" => POOL_FACTORYV1_CONTRACT_ID_CONST.mainnet,
        "testnet" => POOL_FACTORYV1_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near