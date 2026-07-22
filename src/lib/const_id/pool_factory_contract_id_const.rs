// =================================================
/// Contract id constants for the `pool.near` staking-pool factory contract.
pub const POOL_FACTORY_CONTRACT_ID_CONST: POOL_FACTORY_CONTRACT_ID_CONST_TYPE =
    POOL_FACTORY_CONTRACT_ID_CONST_TYPE {
        testnet: "pool.testnet",
        mainnet: "pool.near",
    };
// =================================================
/// String constants for pool.near factory contract ids per network.
pub type POOL_FACTORY_CONTRACT_ID_CONST_TYPE =
    super::contract_id_const_type::CONTRACT_ID_CONST_TYPE;
// =================================================
/// Returns the pool.near factory contract id for the given network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn pool_factory_contractid(network: &str) -> &'static str {
    match network {
        "mainnet" => POOL_FACTORY_CONTRACT_ID_CONST.mainnet,
        "testnet" => POOL_FACTORY_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
