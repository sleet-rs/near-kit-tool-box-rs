// =================================================
/// Contract id constants for the hello.sleet contract.
pub const HELLO_SLEET_CONTRACT_ID_CONST: HELLO_SLEET_CONTRACT_ID_CONST_TYPE =
    HELLO_SLEET_CONTRACT_ID_CONST_TYPE {
        testnet: "hello.sleet.testnet",
        mainnet: "hello.sleet.near",
    };
// =================================================
/// String constants for hello.sleet contract ids per network.
pub struct HELLO_SLEET_CONTRACT_ID_CONST_TYPE {
    pub testnet: &'static str,
    pub mainnet: &'static str,
}
// =================================================
/// Returns the hello.sleet contract id for the given network.
///
/// `network` should be `"testnet"` or `"mainnet"`.
pub fn hello_sleet_contractid_fun(network: &str) -> &'static str {
    match network {
        "mainnet" => HELLO_SLEET_CONTRACT_ID_CONST.mainnet,
        "testnet" => HELLO_SLEET_CONTRACT_ID_CONST.testnet,
        other => panic!("unsupported network `{other}` (use `mainnet` or `testnet`)"),
    }
}
// =================================================
// =================================================
// copyright 2026 by sleet.near