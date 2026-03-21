// get_greeting
//
use near_kit::*;
use near_kit_tool_box::fun::greeting::greeting_get_fun::get_greeting_fun;
// =================================================
#[tokio::main]
async fn main() -> Result<(), Error> {
    let near = Near::testnet().build();
    let contract_id = "hello.sleet.testnet";
    let greeting = get_greeting_fun(&near, contract_id).await?;
    println!("Current Greeting: {greeting}");
    Ok(())
}
// =================================================
// =================================================
// copyright 2026 by sleet.near
