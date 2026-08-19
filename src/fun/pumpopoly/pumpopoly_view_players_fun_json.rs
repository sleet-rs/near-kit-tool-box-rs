// use near_kit::*;
use crate::lib::methods::methods_pumpopoly_world::PUMPOPOLY_WORLD_METHODS_CONST;
use crate::lib::types::pumpopoly_view_player_response_type::PUMPOPOLY_VIEW_PLAYER_RESPONSE_TYPE;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// Fetch multiple Pumpopoly players' current state in a single view call using
/// raw JSON args. Returns one `PUMPOPOLY_VIEW_PLAYER_RESPONSE_TYPE` per requested
/// account, in the same order.
pub async fn view_players(
    near: &Near,
    pumpopoly_contract_id: &str,
    account_ids: &[&str],
) -> Result<Vec<PUMPOPOLY_VIEW_PLAYER_RESPONSE_TYPE>, Error> {
    let accounts: Vec<AccountId> = account_ids
        .iter()
        .map(|a| a.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let players: Vec<PUMPOPOLY_VIEW_PLAYER_RESPONSE_TYPE> = near
        .view::<Vec<PUMPOPOLY_VIEW_PLAYER_RESPONSE_TYPE>>(
            pumpopoly_contract_id,
            PUMPOPOLY_WORLD_METHODS_CONST.view_players,
        )
        .args(json!({ "account_ids": accounts }))
        .await?;
    Ok(players)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
