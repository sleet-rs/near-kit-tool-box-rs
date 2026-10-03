// dclv2_quote (json)
//
// Quotes an exact-input swap across DCL v2 pools via raw JSON args. No
// signer required — only `NEAR_NETWORK` is needed (defaults to testnet).
//
// run:
//   cargo run --bin dclv2_quote_bin_json -- <pool_ids_csv> <input_token> <output_token> <input_amount> [tag] [dcl_id]
//
// example:
//   cargo run --bin dclv2_quote_bin_json -- "wrap.near|usdc.near|2000" wrap.near usdc.near 1000000000000000000000000
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::dclv2::dclv2_quote_fun_json::quote;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let pool_ids: Vec<String> = args
        .get(1)
        .expect("usage: dclv2_quote_bin_json <pool_ids_csv> <input_token> <output_token> <input_amount> [tag] [dcl_id]")
        .split(',')
        .map(str::to_string)
        .collect();
    let input_token = args.get(2).expect("missing <input_token>");
    let output_token = args.get(3).expect("missing <output_token>");
    let input_amount = args.get(4).expect("missing <input_amount>");
    let tag = args.get(5).map(String::as_str);
    let dcl_id = args.get(6).map(String::as_str).unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Quoting swap on DCL v2 contract `{dcl_id}`...");
    let out = quote(&near, dcl_id, &pool_ids, input_token, output_token, input_amount, tag).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
