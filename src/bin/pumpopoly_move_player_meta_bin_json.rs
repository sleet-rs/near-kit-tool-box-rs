// pumpopoly_move_player_meta (meta-tx)
//
// Calls `move_player` on the Pumpopoly world contract via NEAR
// meta-transactions (NEP-366). The signer private key (typically the
// shared `PUMPOPOLY_LIMITED_KEY` function-call key installed on the
// player account) signs a DelegateAction; a separate funded relayer
// account then submits it, paying gas on the user's behalf. The
// contract still sees the original player as the signer.
//
// set in env (.env):
//   NEAR_NETWORK              # testnet (default) or mainnet
//   NEAR_ACCOUNT_ID           # the player account id (the signer)
//   NEAR_PRIVATE_KEY          # the signer private key (the limited key)
//   RELAYER_ACCOUNT_ID        # the relayer account id (must be funded)
//   RELAYER_PRIVATE_KEY       # the relayer's full-access private key
//
// then run:
//   cargo run --bin pumpopoly_move_player_meta_bin_json -- \
//     <pumpopoly_contract_id>
//
// example:
//   cargo run --bin pumpopoly_move_player_meta_bin_json -- \
//     world.pumpopoly.near
//
// =================================================
use near_kit::{Error, Near};
use near_kit_tool_box::fun::pumpopoly::pumpopoly_move_player_meta_fun_json::{
    move_player_meta, submit_move_player_meta,
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
    let contract_id = args.get(1).expect(
        "usage: pumpopoly_move_player_meta_bin_json <pumpopoly_contract_id>",
    );

    let network = env::var("NEAR_NETWORK").unwrap_or_else(|_| "mainnet".to_string());
    let signer_id =
        env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");
    let signer_pk =
        env::var("NEAR_PRIVATE_KEY").expect("NEAR_PRIVATE_KEY env var is required");
    let relayer_id =
        env::var("RELAYER_ACCOUNT_ID").expect("RELAYER_ACCOUNT_ID env var is required");
    let relayer_pk =
        env::var("RELAYER_PRIVATE_KEY").expect("RELAYER_PRIVATE_KEY env var is required");

    let signer = build_near(&network, &signer_pk, &signer_id);
    let relayer = build_near(&network, &relayer_pk, &relayer_id);

    println!(
        "Submitting move_player meta-tx on `{}` for `{}` via relayer `{}`...",
        contract_id, signer_id, relayer_id
    );

    let delegate = move_player_meta(&signer, contract_id).await?;
    let result = submit_move_player_meta(&relayer, delegate).await?;
    println!("✅ {} tx: {}", signer_id, result.transaction.hash);

    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
