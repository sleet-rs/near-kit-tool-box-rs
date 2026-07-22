// wrap_near_withdraw (json)
//
// Unwraps wNEAR back into NEAR via the wrap.near / wrap.testnet
// contract using raw JSON args.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin wrap_near_withdraw_bin_json -- <wrap_contract_id> <amount_yocto>
//
// example:
//   cargo run --bin wrap_near_withdraw_bin_json -- wrap.testnet 1000000000000000000000000
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::near::wrap_near_withdraw_fun_json::wrap_near_withdraw;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let wrap_contract_id = args
        .get(1)
        .expect("usage: wrap_near_withdraw_bin_json <wrap_contract_id> <amount_yocto>");
    let amount = args
        .get(2)
        .expect("usage: wrap_near_withdraw_bin_json <wrap_contract_id> <amount_yocto>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();

    println!(
        "Unwrapping {} yoctoNEAR from `{}`...",
        amount, wrap_contract_id
    );
    let result = wrap_near_withdraw(&near, wrap_contract_id, amount).await?;
    println!("✅ Unwrap complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
