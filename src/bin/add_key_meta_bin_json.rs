// add_key_meta (meta-tx)
//
// Adds an access key to one or more accounts via NEAR meta-transactions
// (NEP-366). The shared user private key — already a full-access key on
// every target account — signs a DelegateAction per target; a separate
// funded relayer account then submits each one, paying gas (and any
// other related costs) on the target's behalf. Works even on accounts
// with zero own balance — no need to fund them just to add a key.
//
// set in env (.env):
//   NEAR_NETWORK              # testnet (default) or mainnet
//   NEAR_PRIVATE_KEY          # the shared full-access private key
//                             # already on every target account
//   RELAYER_ACCOUNT_ID        # the relayer account id (must be funded)
//   RELAYER_PRIVATE_KEY       # the relayer's full-access private key
//
// then run:
//   cargo run --bin add_key_meta_bin_json -- \
//     <new_public_key> <account_1> [account_2 ...]
//
// example:
//   cargo run --bin add_key_meta_bin_json -- \
//     ed25519:2eDMWnKcDt7UQ1xVximcWbd1YKwJbfE7HGPmNZSegjcV \
//     walcwarchest.near walcpool.near mattbwalc.near
//
// =================================================
use near_kit::{AccessKeyPermission, Error, Near};
use near_kit_tool_box::fun::near::add_key_meta_fun_json::{
    sign_add_key_meta, submit_add_key_meta,
};
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use std::env;
// =================================================
fn build_near(network: &str, pk: &str, account_id: &str) -> Near {
    let base = if network == "mainnet" {
        NEAR_KIT_CLIENT::mainnet()
    } else {
        NEAR_KIT_CLIENT::testnet()
    };
    base.credentials(pk, account_id)
        .expect("invalid NEAR credentials")
        .build()
}
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let public_key = args
        .get(1)
        .expect("usage: add_key_meta_bin_json <new_public_key> <account_1> [account_2 ...]");
    let accounts: Vec<String> = args[2..].to_vec();
    if accounts.is_empty() {
        panic!(
            "usage: add_key_meta_bin_json <new_public_key> <account_1> [account_2 ...] (at least one target account is required)"
        );
    }

    let network = env::var("NEAR_NETWORK").unwrap_or_else(|_| "testnet".to_string());
    let user_private_key =
        env::var("NEAR_PRIVATE_KEY").expect("NEAR_PRIVATE_KEY env var is required");
    let relayer_id =
        env::var("RELAYER_ACCOUNT_ID").expect("RELAYER_ACCOUNT_ID env var is required");
    let relayer_pk =
        env::var("RELAYER_PRIVATE_KEY").expect("RELAYER_PRIVATE_KEY env var is required");

    let relayer = build_near(&network, &relayer_pk, &relayer_id);

    println!(
        "Adding access key `{}` to {} account(s) via relayer `{}`...",
        public_key,
        accounts.len(),
        relayer_id
    );

    for account_id in &accounts {
        let user = build_near(&network, &user_private_key, account_id);
        let delegate = sign_add_key_meta(
            &user,
            account_id,
            public_key,
            AccessKeyPermission::full_access(),
        )
        .await?;
        let result = submit_add_key_meta(&relayer, delegate).await?;
        println!("✅ {} tx: {}", account_id, result.transaction.hash);
    }

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
