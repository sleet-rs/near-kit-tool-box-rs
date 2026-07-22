// ft_transfer_call (json)
//
// Transfers FT to `receiver_id` and calls `ft_transfer_call` on it
// with `msg` via raw JSON args.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin ft_transfer_call_bin_json -- <ft_contract_id> <receiver_id> <amount> <msg>
//
// example:
//   cargo run --bin ft_transfer_call_bin_json -- wrap.testnet alice.testnet 1000000000000000000000000 "register"
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::ft::ft_transfer_call_fun_json::ft_transfer_call;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let ft_contract_id = args
        .get(1)
        .expect("usage: ft_transfer_call_bin_json <ft_contract_id> <receiver_id> <amount> <msg>");
    let receiver_id = args
        .get(2)
        .expect("usage: ft_transfer_call_bin_json <ft_contract_id> <receiver_id> <amount> <msg>");
    let amount = args
        .get(3)
        .expect("usage: ft_transfer_call_bin_json <ft_contract_id> <receiver_id> <amount> <msg>");
    let msg = args
        .get(4)
        .expect("usage: ft_transfer_call_bin_json <ft_contract_id> <receiver_id> <amount> <msg>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();
    let account_id = env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");

    println!(
        "Transferring {} of `{}` from `{}` to `{}` with msg `{}`...",
        amount, ft_contract_id, account_id, receiver_id, msg
    );
    let result = ft_transfer_call(&near, ft_contract_id, receiver_id, amount, msg).await?;
    println!(
        "✅ Transfer call complete. tx id: {}",
        result.transaction.hash
    );
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
