// nft_tokens_for_owner (json)
//
// Lists the Pumpopoly NFTs an account holds, with full metadata, from the
// mainnet NFT contract. No signer required.
//
// `from_index` and `limit` are optional and paginate the result. Keep the
// limit modest (~50) — each token carries a full metadata blob.
//
// run:
//   cargo run --bin pumpopoly_nft_tokens_for_owner_bin_json -- <account_id> [from_index] [limit]
//
// example:
//   cargo run --bin pumpopoly_nft_tokens_for_owner_bin_json -- sleet.near 0 5
//
// =================================================
use near_kit::Error;
use near_kit_tool_box::fun::pumpopoly_nft::pumpopoly_nft_tokens_for_owner_fun_json::nft_tokens_for_owner;
use near_kit_tool_box::lib::client_kit::NEAR_KIT_CLIENT;
use near_kit_tool_box::lib::const_id::pumpopoly_nft_contract_id_const::PUMPOPOLY_NFT_CONTRACT_ID_CONST;
use std::env;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let account_id = args
        .get(1)
        .expect("usage: pumpopoly_nft_tokens_for_owner_bin_json <account_id> [from_index] [limit]");
    let from_index = args.get(2).map(|s| s.as_str());
    let limit = args
        .get(3)
        .map(|s| s.parse::<u64>().expect("limit must be a number"));

    let near = NEAR_KIT_CLIENT::mainnet().build();

    println!("Fetching Pumpopoly NFTs held by `{}`...", account_id);
    let tokens = nft_tokens_for_owner(
        &near,
        PUMPOPOLY_NFT_CONTRACT_ID_CONST,
        account_id,
        from_index,
        limit,
    )
    .await?;
    println!("Got {} token(s):", tokens.len());
    for token in &tokens {
        let title = token
            .metadata
            .as_ref()
            .and_then(|m| m.title.as_deref())
            .unwrap_or("(no title)");
        println!("  {} — {}", token.token_id, title);
    }
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
