use near_kit::Near;
// =================================================
pub fn print_client_details_fun(near: &Near) {
    let account_id = near
        .try_account_id()
        .map(ToString::to_string)
        .unwrap_or_else(|| "none (read-only)".to_string());
    let mode = if near.signer().is_some() {
        "signing"
    } else {
        "read-only"
    };

    println!("==========================");
    println!("NEAR client details:");
    println!("  Network: {}", near.chain_id());
    println!("  RPC URL: {}", near.rpc_url());
    println!("  Account ID: {account_id}");
    println!("  Mode: {mode}");
    if let Some(public_key) = near.public_key() {
        println!("  Public key: {public_key}");
    }
    println!("==========================");
}
// =================================================
