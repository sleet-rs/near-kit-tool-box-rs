// rhea_dclv2_quote_by_output (json)
//
// Quotes an exact-output swap across DCL v2 pools via raw JSON args. No
// signer required — only `NEAR_KIT_NETWORK` is needed.
//
// set NEAR_KIT_NETWORK, NEAR_KIT_ACCOUNT_ID, NEAR_KIT_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin rhea_dclv2_quote_by_output_bin_json -- <pool_ids_csv> <input_token> <output_token> <output_amount> [dcl_id]
//
// example:
//   cargo run --bin rhea_dclv2_quote_by_output_bin_json -- "wrap.near|usdc.near|2000" wrap.near usdc.near 1000000
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_quote_by_output_fun_json::quote_by_output;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let pool_ids: Vec<String> = args
        .get(1)
        .expect("usage: rhea_dclv2_quote_by_output_bin_json <pool_ids_csv> <input_token> <output_token> <output_amount> [dcl_id]")
        .split(',')
        .map(str::to_string)
        .collect();
    let input_token = args.get(2).expect("missing <input_token>");
    let output_token = args.get(3).expect("missing <output_token>");
    let output_amount = args.get(4).expect("missing <output_amount>");
    let dcl_id = args
        .get(5)
        .map(String::as_str)
        .unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Quoting exact-output swap on DCL v2 contract `{dcl_id}`...");
    let out = quote_by_output(
        &near,
        dcl_id,
        &pool_ids,
        input_token,
        output_token,
        output_amount,
    )
    .await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
