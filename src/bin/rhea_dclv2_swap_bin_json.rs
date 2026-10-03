// rhea_dclv2_swap (json)
//
// Swaps on DCL v2 via ft_transfer_call on the INPUT token (DCL has no
// direct swap method) using raw JSON args.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin rhea_dclv2_swap_bin_json -- <token_in> <amount_in> <pool_ids_csv> <output_token> <min_output> [dcl_id]
//
// example:
//   cargo run --bin rhea_dclv2_swap_bin_json -- usdc.fakes.testnet 1000000 "usdc.fakes.testnet|wrap.testnet|2000" wrap.testnet 0 dclv2.ref-dev.testnet
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_swap_ft_transfer_call_fun_json::swap;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let token_in = args.get(1).expect("usage: rhea_dclv2_swap_bin_json <token_in> <amount_in> <pool_ids_csv> <output_token> <min_output> [dcl_id]");
    let amount_in = args.get(2).expect("amount_in required");
    let pool_ids: Vec<String> = args
        .get(3)
        .expect("pool_ids_csv required")
        .split(',')
        .map(str::to_string)
        .collect();
    let output_token = args.get(4).expect("output_token required");
    let min_output = args.get(5).expect("min_output required");
    let dcl_id = args.get(6).map(String::as_str).unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Swapping `{amount_in}` of `{token_in}` via `{dcl_id}` pools {pool_ids:?}...");
    let result = swap(&near, token_in, amount_in, dcl_id, &pool_ids, output_token, min_output).await?;
    println!("✅ swap complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
