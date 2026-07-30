// use near_kit::*;
use crate::lib::methods::methods_pumpopoly_world::PUMPOPOLY_WORLD_METHODS_CONST;
use near_kit::{Error, Gas, Near};
use serde_json::json;
// =================================================
/// Perform an elite move for the signer in the Pumpopoly world using raw JSON
/// args.
///
/// Requires a `Near` client configured with signer credentials and returns the
/// final execution outcome.
pub async fn elite_move(
    near: &Near,
    pumpopoly_contract_id: &str,
) -> Result<near_kit::FinalExecutionOutcome, Error> {
    let result = near
        .call(
            pumpopoly_contract_id,
            PUMPOPOLY_WORLD_METHODS_CONST.elite_move,
        )
        .args(json!({ "contract_id": "1" }))
        .gas(Gas::from_tgas(100))
        .await?;
    Ok(result)
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
