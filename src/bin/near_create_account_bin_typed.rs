// near_create_account (typed)
//
// Creates a new sub-account via the TLD registrar contract (e.g.
// `near` on mainnet, `testnet` on testnet) using the typed contract
// interface. The TLD contract id is resolved automatically from
// `NEAR_NETWORK`.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin near_create_account_bin_typed -- <new_account_id> <new_public_key>
//
// example:
//   cargo run --bin near_create_account_bin_typed -- newaccounttests.testnet ed25519:56CJx1YrTJ6nBmJb8DoXwBm5HLHZcoyAt8FQDmcvkjFE
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::near::near_create_account_fun_typed::near_create_account_typed;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let new_account_id = args
        .get(1)
        .expect("usage: near_create_account_bin_typed <new_account_id> <new_public_key>");
    let new_public_key = args
        .get(2)
        .expect("usage: near_create_account_bin_typed <new_account_id> <new_public_key>");

    let near = NEAR_KIT_CLIENT::from_env()?.build();

    println!("Creating sub-account `{}`...", new_account_id);

    let result = near_create_account_typed(&near, new_account_id, new_public_key).await?;

    println!("✅ Created sub-account. tx id: {}", result.transaction.hash);

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near