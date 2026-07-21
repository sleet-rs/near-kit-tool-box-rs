// =================================================
/// Contract id constants for the `pool.near` staking-pool factory contract.
pub const POOL_FACTORY_CONTRACT_ID_CONST: POOL_FACTORY_CONTRACT_ID_CONST_TYPE =
    POOL_FACTORY_CONTRACT_ID_CONST_TYPE {
        testnet: "pool.testnet",
        mainnet: "pool.near",
    };
// =================================================
/// String constants for pool.near factory contract ids per network.
pub struct POOL_FACTORY_CONTRACT_ID_CONST_TYPE {
    pub testnet: &'static str,
    pub mainnet: &'static str,
}
// =================================================
/// Returns the pool.near factory contract id for the given network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn pool_factory_contractid_fun(network: &str) -> &'static str {
    match network {
        "mainnet" => POOL_FACTORY_CONTRACT_ID_CONST.mainnet,
        "testnet" => POOL_FACTORY_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near