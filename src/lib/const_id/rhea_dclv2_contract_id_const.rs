// =================================================
/// Contract id constants for the DCL v2 (discrete concentrated liquidity) contract.
pub const RHEA_DCLV2_CONTRACT_ID_CONST: RHEA_DCLV2_CONTRACT_ID_CONST_TYPE = RHEA_DCLV2_CONTRACT_ID_CONST_TYPE {
    testnet: "dclv2.ref-dev.testnet",
    mainnet: "dclv2.ref-labs.near",
};
// =================================================
/// String constants for the DCL v2 DEX contract ids per network.
pub type RHEA_DCLV2_CONTRACT_ID_CONST_TYPE = super::contract_id_const_type::CONTRACT_ID_CONST_TYPE;
// =================================================
/// Returns the DCL v2 contract id for the given network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn rhea_dclv2_contractid(network: &str) -> &'static str {
    match network {
        "mainnet" => RHEA_DCLV2_CONTRACT_ID_CONST.mainnet,
        "testnet" => RHEA_DCLV2_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
