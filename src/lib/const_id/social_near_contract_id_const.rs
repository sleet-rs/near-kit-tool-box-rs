// =================================================
/// Contract id constants for the social.near social-db contract.
pub const SOCIAL_NEAR_CONTRACT_ID_CONST: SOCIAL_NEAR_CONTRACT_ID_CONST_TYPE =
    SOCIAL_NEAR_CONTRACT_ID_CONST_TYPE {
        testnet: "v1.social08.testnet",
        mainnet: "social.near",
    };
// =================================================
/// String constants for social.near contract ids per network.
pub type SOCIAL_NEAR_CONTRACT_ID_CONST_TYPE = super::contract_id_const_type::CONTRACT_ID_CONST_TYPE;
// =================================================
/// Returns the social.near contract id for the given network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn social_near_contractid_fun(network: &str) -> &'static str {
    match network {
        "mainnet" => SOCIAL_NEAR_CONTRACT_ID_CONST.mainnet,
        "testnet" => SOCIAL_NEAR_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
