// dclv2_get_user_asset (json)
//
// Reads one inner token balance an account holds inside the DCL v2
// contract via raw JSON args. No signer required.
//
// run:
//   cargo run --bin dclv2_get_user_asset_bin_json -- <account_id> <token_id> [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::dclv2::dclv2_get_user_asset_fun_json::get_user_asset;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: dclv2_get_user_asset_bin_json <account_id> <token_id> [dcl_id]");
    let token_id = args.get(2).expect("missing <token_id>");
    let dcl_id = args.get(3).map(String::as_str).unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching `{token_id}` balance of `{account_id}` via `{dcl_id}`...");
    let out = get_user_asset(&near, dcl_id, account_id, token_id).await?;
    println!("{out:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
