// dclv2_withdraw_asset (json)
//
// Withdraws an internal token balance out of DCL v2 via raw JSON args.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin dclv2_withdraw_asset_bin_json -- <token_id> <amount> [dcl_id]
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::dclv2::dclv2_withdraw_asset_fun_json::withdraw_asset;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let token_id = args.get(1).expect("usage: dclv2_withdraw_asset_bin_json <token_id> [amount] [dcl_id]");
    // Disambiguate: contract ids contain '.', amounts are digits only.
    let (amount, dcl_id): (Option<&str>, &str) = match (args.get(2), args.get(3)) {
        (Some(second), Some(third)) => (Some(second.as_str()), third.as_str()),
        (Some(second), None) if second.contains('.') => (None, second.as_str()),
        (Some(second), None) => (Some(second.as_str()), "dclv2.ref-dev.testnet"),
        (None, _) => panic!("usage: dclv2_withdraw_asset_bin_json <token_id> [amount] [dcl_id]"),
    };

    let near = NEAR_KIT_CLIENT::from_env()?;

    println!("Withdrawing `{token_id}` amount {amount:?} via `{dcl_id}`...");
    let result = withdraw_asset(&near, dcl_id, token_id, amount).await?;
    println!("✅ withdraw_asset complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
