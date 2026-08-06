// nft_transfer (json)
//
// Transfers a Pumpopoly NFT to `receiver_id` on the mainnet NFT contract.
//
// Token ids contain spaces, so quote them. `memo` is an optional note
// recorded in the transfer event.
//
// set NEAR_ACCOUNT_ID, NEAR_PRIVATE_KEY in env (.env) then run:
//   cargo run --bin pumpopoly_nft_transfer_bin_json -- <receiver_id> <token_id> [memo]
//
// example:
//   cargo run --bin pumpopoly_nft_transfer_bin_json -- alice.near "Bond 10-849-1"
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::pumpopoly_nft::pumpopoly_nft_transfer_fun_json::nft_transfer;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use near_kit_tool_box::lib::const_id::pumpopoly_nft_contract_id_const::PUMPOPOLY_NFT_CONTRACT_ID_CONST;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let receiver_id = args
        .get(1)
        .expect("usage: pumpopoly_nft_transfer_bin_json <receiver_id> <token_id> [memo]");
    let token_id = args
        .get(2)
        .expect("usage: pumpopoly_nft_transfer_bin_json <receiver_id> <token_id> [memo]");
    let memo = args.get(3).map(|s| s.as_str());

    // pinned to mainnet — nft.pumpopoly.near has no testnet deployment
    let account_id = env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID env var is required");
    let private_key = env::var("NEAR_PRIVATE_KEY").expect("NEAR_PRIVATE_KEY env var is required");
    let near = NEAR_KIT_CLIENT::mainnet()
        .credentials(&private_key, &account_id)?
        .build();

    println!(
        "Transferring Pumpopoly NFT `{}` from `{}` to `{}`...",
        token_id, account_id, receiver_id
    );
    let result = nft_transfer(
        &near,
        PUMPOPOLY_NFT_CONTRACT_ID_CONST,
        receiver_id,
        token_id,
        None,
        memo,
    )
    .await?;
    println!("✅ Transfer complete. tx id: {}", result.transaction.hash);
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
