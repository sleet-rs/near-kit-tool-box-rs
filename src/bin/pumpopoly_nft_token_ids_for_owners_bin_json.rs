// nft_token_ids_for_owners (json)
//
// Lists just the Pumpopoly NFT token ids held by one or more accounts on
// the mainnet NFT contract. No signer required.
//
// Cheaper than `nft_tokens_for_owner` — no metadata, no pagination, so a
// full 100+ token wallet comes back in a single call.
//
// run:
//   cargo run --bin pumpopoly_nft_token_ids_for_owners_bin_json -- <account_id> [account_id...]
//
// example:
//   cargo run --bin pumpopoly_nft_token_ids_for_owners_bin_json -- sleet.near
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::pumpopoly_nft::pumpopoly_nft_token_ids_for_owners_fun_json::nft_token_ids_for_owners;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use near_kit_tool_box::lib::const_id::pumpopoly_nft_contract_id_const::PUMPOPOLY_NFT_CONTRACT_ID_CONST;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_ids: Vec<&str> = args.iter().skip(1).map(|s| s.as_str()).collect();
    if account_ids.is_empty() {
        panic!("usage: pumpopoly_nft_token_ids_for_owners_bin_json <account_id> [account_id...]");
    }

    let near = NEAR_KIT_CLIENT::mainnet().build();

    println!("Fetching Pumpopoly NFT token ids for {:?}...", account_ids);
    let by_owner =
        nft_token_ids_for_owners(&near, PUMPOPOLY_NFT_CONTRACT_ID_CONST, &account_ids).await?;
    for account_id in &account_ids {
        match by_owner.get(*account_id) {
            Some(token_ids) => {
                println!("{} — {} token(s):", account_id, token_ids.len());
                for token_id in token_ids {
                    println!("  {}", token_id);
                }
            }
            None => println!("{} — no tokens", account_id),
        }
    }
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
