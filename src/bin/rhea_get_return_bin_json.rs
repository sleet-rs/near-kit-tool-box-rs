// rhea_get_return (json)
//
// Simulates a swap on the rhea / ref-finance DEX via raw JSON args,
// returning the expected amount of `token_out` you'd receive for
// `amount_in` of `token_in` on `pool_id`. No signer required — only
// `NEAR_NETWORK` is needed (defaults to testnet).
//
// run:
//   cargo run --bin rhea_get_return_bin_json -- <pool_id> <token_in> <amount_in> <token_out> [rhea_contract_id]
//
// `amount_in` is in yocto units of `token_in` (10^decimals).
//
// example (mainnet wNEAR/USDC.e pool 2794 — 0.01 wNEAR in):
//   cargo run --bin rhea_get_return_bin_json -- \
//     2794 wrap.near 10000000000000000000000 a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.near v2.ref-finance.near
//
// example (testnet, check a live pool for current tokens + pool_id first):
//   cargo run --bin rhea_get_return_bin_json -- 0 wrap.testnet 10000000000000000000000 wrap.testnet ref-finance-101.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea::rhea_get_return_fun_json::get_return;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let pool_id: u32 = args
        .get(1)
        .expect("usage: rhea_get_return_bin_json <pool_id> <token_in> <amount_in> <token_out> [rhea_contract_id]")
        .parse()
        .expect("pool_id must be a u32");
    let token_in = args
        .get(2)
        .expect("usage: rhea_get_return_bin_json <pool_id> <token_in> <amount_in> <token_out> [rhea_contract_id]");
    let amount_in = args
        .get(3)
        .expect("usage: rhea_get_return_bin_json <pool_id> <token_in> <amount_in> <token_out> [rhea_contract_id]");
    let token_out = args
        .get(4)
        .expect("usage: rhea_get_return_bin_json <pool_id> <token_in> <amount_in> <token_out> [rhea_contract_id]");
    let rhea_contract_id = args
        .get(5)
        .map(String::as_str)
        .unwrap_or("ref-finance-101.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!(
        "Quoting swap on rhea contract `{}` pool `{}`: {} {} -> {} ...",
        rhea_contract_id, pool_id, amount_in, token_in, token_out
    );
    let amount_out = get_return(&near, rhea_contract_id, pool_id, token_in, amount_in, token_out).await?;
    println!("Out: {amount_out}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
