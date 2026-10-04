// =================================================
/// Contract id constants for the rhea_ammv2 / ref-finance DEX contract.
pub const RHEA_AMMV2_CONTRACT_ID_CONST: RHEA_AMMV2_CONTRACT_ID_CONST_TYPE = RHEA_AMMV2_CONTRACT_ID_CONST_TYPE {
    testnet: "ref-finance-101.testnet",
    mainnet: "v2.ref-finance.near",
};
// =================================================
/// String constants for the rhea_ammv2 / ref-finance DEX contract ids per network.
pub type RHEA_AMMV2_CONTRACT_ID_CONST_TYPE = super::contract_id_const_type::CONTRACT_ID_CONST_TYPE;
// =================================================
/// Returns the rhea_ammv2 / ref-finance contract id for the given network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn rhea_ammv2_contractid(network: &str) -> &'static str {
    match network {
        "mainnet" => RHEA_AMMV2_CONTRACT_ID_CONST.mainnet,
        "testnet" => RHEA_AMMV2_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
