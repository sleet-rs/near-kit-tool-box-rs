// use near_kit::*;
use crate::lib::methods::methods_pumpopoly_world::PUMPOPOLY_WORLD_METHODS_CONST;
use crate::lib::types::pumpopoly_view_player_response_type::PUMPOPOLY_VIEW_PLAYER_RESPONSE_TYPE;
use near_kit::{AccountId, Error, Near};
use serde_json::json;
// =================================================
/// Fetch a Pumpopoly player's current state using raw JSON args.
pub async fn view_player(
    near: &Near,
    pumpopoly_contract_id: &str,
    account_id: &str,
) -> Result<PUMPOPOLY_VIEW_PLAYER_RESPONSE_TYPE, Error> {
    let account: AccountId = account_id.parse()?;
    let player: PUMPOPOLY_VIEW_PLAYER_RESPONSE_TYPE = near
        .view::<PUMPOPOLY_VIEW_PLAYER_RESPONSE_TYPE>(
            pumpopoly_contract_id,
            PUMPOPOLY_WORLD_METHODS_CONST.view_player,
        )
        .args(json!({ "account_id": account }))
        .await?;
    Ok(player)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
