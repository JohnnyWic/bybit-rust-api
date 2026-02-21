//! Example: Subscribe to orderbook via WebSocket

use bybit_api::websocket::BybitWebSocket;
use bybit_api::TESTNET_WS_PUBLIC_LINEAR;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> bybit_api::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create public WebSocket client for testnet
    let mut ws = BybitWebSocket::public(TESTNET_WS_PUBLIC_LINEAR);

    // Connect
    println!("Connecting to WebSocket...");
    ws.connect().await?;

    // Subscribe to orderbook
    println!("Subscribing to BTCUSDT orderbook...");
    ws.subscribe(vec!["orderbook.50.BTCUSDT".to_string()], |msg| {
        println!("Received orderbook update:");
        println!("  Topic: {}", msg.topic);
        println!("  Type: {:?}", msg.msg_type);

        if let Some(data) = msg.data.as_object() {
            if let Some(bids) = data.get("b").and_then(|v| v.as_array()) {
                println!("  Top 3 bids:");
                for bid in bids.iter().take(3) {
                    if let Some(arr) = bid.as_array() {
                        println!(
                            "    {} @ {}",
                            arr.get(1).unwrap_or(&serde_json::Value::Null),
                            arr.first().unwrap_or(&serde_json::Value::Null)
                        );
                    }
                }
            }
            if let Some(asks) = data.get("a").and_then(|v| v.as_array()) {
                println!("  Top 3 asks:");
                for ask in asks.iter().take(3) {
                    if let Some(arr) = ask.as_array() {
                        println!(
                            "    {} @ {}",
                            arr.get(1).unwrap_or(&serde_json::Value::Null),
                            arr.first().unwrap_or(&serde_json::Value::Null)
                        );
                    }
                }
            }
        }
        println!();
    })
    .await?;

    // Keep running for 30 seconds
    println!("Listening for updates (30 seconds)...\n");
    sleep(Duration::from_secs(30)).await;

    // Disconnect
    ws.disconnect().await?;
    println!("Disconnected.");

    Ok(())
}
