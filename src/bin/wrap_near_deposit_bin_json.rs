// wrap_near_deposit (json)
//
// Wraps NEAR into wNEAR via the wrap.near / wrap.testnet contract
// using raw JSON args.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin wrap_near_deposit_bin_json -- <wrap_contract_id> <amount>
//
// example:
//   cargo run --bin wrap_near_deposit_bin_json -- wrap.testnet "1 NEAR"
//
// =================================================
use near_kit::{Error, IntoNearToken, NearToken};
use near_kit_tool_box::fun::near::wrap_near_deposit_fun_json::wrap_near_deposit;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let wrap_contract_id = args
        .get(1)
        .expect("usage: wrap_near_deposit_bin_json <wrap_contract_id> <amount>");
    let amount: NearToken = args
        .get(2)
        .expect("usage: wrap_near_deposit_bin_json <wrap_contract_id> <amount>")
        .as_str()
        .into_near_token()?;

    let near = NEAR_KIT_CLIENT::from_env()?.build();
    let account_id = env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");

    println!(
        "Wrapping {} into wNEAR via `{}` for `{}`...",
        amount, wrap_contract_id, account_id
    );
    let result = wrap_near_deposit(&near, wrap_contract_id, amount).await?;
    println!("✅ Wrap complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
