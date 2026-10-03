// rhea_dclv2_deposit (json)
//
// Funds the signer's DCL v2 internal balance via ft_transfer_call with an
// empty msg, using raw JSON args. Check with rhea_dclv2_get_user_asset_bin_json.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin rhea_dclv2_deposit_bin_json -- <token_id> <amount> [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::rhea_dclv2::dclv2_deposit_fun_json::deposit;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let token_id = args.get(1).expect("usage: rhea_dclv2_deposit_bin_json <token_id> <amount> [dcl_id]");
    let amount = args.get(2).expect("usage: rhea_dclv2_deposit_bin_json <token_id> <amount> [dcl_id]");
    let dcl_id = args.get(3).map(String::as_str).unwrap_or("dclv2.ref-dev.testnet");

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Depositing `{amount}` of `{token_id}` into `{dcl_id}`...");
    let result = deposit(&near, token_id, amount, dcl_id).await?;
    println!("✅ deposit complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
