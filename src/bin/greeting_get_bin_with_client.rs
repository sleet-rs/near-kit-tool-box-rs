// get_greeting
//
use near_kit::Error;
use near_kit_tool_box::fun::greeting::greeting_get_fun::get_greeting_fun;
use near_kit_tool_box::lib::client_kit::NearClient;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let near = NearClient::testnet().build();
    let contract_id = "hello.sleet.testnet";
    let greeting = get_greeting_fun(&near, contract_id).await?;
    println!("Current Greeting: {greeting}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
