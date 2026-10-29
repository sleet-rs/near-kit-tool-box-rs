use std::collections::HashMap;
// =================================================
/// Internal balance one account holds of one token inside the DCL v2
/// contract.
///
/// The live `get_user_asset` response on `dclv2.ref-labs.near` is a bare
/// decimal string such as `"0"`, not an object. The account and token are
/// both in the request, so the response does not repeat them.
pub type RHEA_DCLV2_USER_ASSET_TYPE = String;

/// Internal balances of every token an account holds inside the DCL v2
/// contract.
///
/// The `list_user_assets` response is a `{ token_id: balance }` map, with
/// each balance a decimal string in the token's smallest unit.
pub type RHEA_DCLV2_USER_ASSETS_TYPE = HashMap<String, String>;
// =================================================
// =================================================
// copyright 2026 by sleet.near
