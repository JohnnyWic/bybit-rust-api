//! Example: Subscribe to private channels via WebSocket

use bybit_api::websocket::BybitWebSocket;
use bybit_api::TESTNET_WS_PRIVATE;
use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> bybit_api::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get API credentials from environment
    let api_key = env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY environment variable not set");
    let api_secret =
        env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET environment variable not set");

    // Create private WebSocket client for testnet
    let mut ws = BybitWebSocket::private(&api_key, &api_secret, TESTNET_WS_PRIVATE);

    // Connect (will automatically authenticate)
    println!("Connecting to private WebSocket...");
    ws.connect().await?;

    // Wait for auth
    sleep(Duration::from_secs(2)).await;

    // Subscribe to private channels
    println!("Subscribing to private channels...");
    ws.subscribe(
        vec![
            "position".to_string(),
            "order".to_string(),
            "wallet".to_string(),
        ],
        |msg| {
            println!("Received private update:");
            println!("  Topic: {}", msg.topic);
            println!(
                "  Data: {}",
                serde_json::to_string_pretty(&msg.data).unwrap_or_default()
            );
            println!();
        },
    )
    .await?;

    // Keep running for 60 seconds
    println!("Listening for updates (60 seconds)...\n");
    println!("Try placing an order in another terminal to see updates.\n");
    sleep(Duration::from_secs(60)).await;

    // Disconnect
    ws.disconnect().await?;
    println!("Disconnected.");

    Ok(())
}
