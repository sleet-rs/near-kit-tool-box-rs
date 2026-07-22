// near_create_account (json)
//
// Creates a new sub-account via the `near` / `testnet` TLD registrar
// contract using raw JSON args.
//
// set NEAR_NETWORK, NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env)
// then run:
//   cargo run --bin near_create_account_bin_json -- <tld_contract_id> <new_account_id> <new_public_key>
//
// example:
//   cargo run --bin near_create_account_bin_json -- testnet newaccounttests.testnet ed25519:56CJx1YrTJ6nBmJb8DoXwBm5HLHZcoyAt8FQDmcvkjFE
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::near::near_create_account_fun_json::near_create_account;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let tld_contract_id = args.get(1).expect(
        "usage: near_create_account_bin_json <tld_contract_id> <new_account_id> <new_public_key>",
    );
    let new_account_id = args.get(2).expect(
        "usage: near_create_account_bin_json <tld_contract_id> <new_account_id> <new_public_key>",
    );
    let new_public_key = args.get(3).expect(
        "usage: near_create_account_bin_json <tld_contract_id> <new_account_id> <new_public_key>",
    );

    let near = NEAR_KIT_CLIENT::from_env()?.build();

    println!(
        "Creating sub-account `{}` via TLD `{}`...",
        new_account_id, tld_contract_id
    );

    let result =
        near_create_account(&near, tld_contract_id, new_account_id, new_public_key).await?;

    println!("✅ Created sub-account. tx id: {}", result.transaction.hash);

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
