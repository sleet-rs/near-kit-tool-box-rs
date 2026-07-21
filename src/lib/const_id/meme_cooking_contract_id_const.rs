// =================================================
/// Contract id constants for the meme-cooking meme launchpad contract.
pub const MEME_COOKING_CONTRACT_ID_CONST: MEME_COOKING_CONTRACT_ID_CONST_TYPE =
    MEME_COOKING_CONTRACT_ID_CONST_TYPE {
        testnet: "factory.v10.meme-cooking.testnet",
        mainnet: "meme-cooking.near",
    };
// =================================================
/// String constants for meme-cooking contract ids per network.
pub type MEME_COOKING_CONTRACT_ID_CONST_TYPE = super::contract_id_const_type::CONTRACT_ID_CONST_TYPE;
// =================================================
/// Returns the meme-cooking contract id for the given network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn meme_cooking_contractid_fun(network: &str) -> &'static str {
    match network {
        "mainnet" => MEME_COOKING_CONTRACT_ID_CONST.mainnet,
        "testnet" => MEME_COOKING_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near