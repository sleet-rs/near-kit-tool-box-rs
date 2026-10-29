// rhea_dclv2_get_metadata (json)
//
// Reads the DCL v2 contract metadata via raw JSON args. No signer
// required — only `NEAR_KIT_NETWORK` is needed (defaults to testnet).
//
// run:
//   cargo run --bin rhea_dclv2_get_metadata_bin_json -- [dcl_id]
//
// example:
//   cargo run --bin rhea_dclv2_get_metadata_bin_json -- dclv2.ref-labs.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_get_metadata_fun_json::get_metadata;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let dcl_id = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("dclv2.ref-labs.near");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Fetching DCL v2 metadata from contract `{dcl_id}`...");
    let metadata = get_metadata(&near, dcl_id).await?;
    println!("{metadata:#?}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
