//! Example: Get positions

use bybit_api::{BybitClient, Category};
use std::env;

#[tokio::main]
async fn main() -> bybit_api::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get API credentials from environment
    let api_key = env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY environment variable not set");
    let api_secret =
        env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET environment variable not set");

    // Create client for testnet
    let client = BybitClient::testnet(&api_key, &api_secret)?;

    // Get all linear positions (need to specify settleCoin)
    println!("Fetching positions...");
    let positions = client.get_positions(Category::Linear, None, Some("USDT")).await?;

    println!("Found {} positions", positions.list.len());

    for pos in &positions.list {
        // Only show positions with size > 0
        if pos.size != "0" && !pos.size.is_empty() {
            println!("\n{} - {} {}", pos.symbol, pos.side, pos.size);
            println!("  Entry Price: {}", pos.avg_price);
            println!("  Mark Price: {}", pos.mark_price);
            println!("  Liq Price: {}", pos.liq_price);
            println!("  Unrealised PnL: {}", pos.unrealised_pnl);
            println!("  Leverage: {}x", pos.leverage);
            println!("  TP: {} / SL: {}", pos.take_profit, pos.stop_loss);
        }
    }

    if positions
        .list
        .iter()
        .all(|p| p.size == "0" || p.size.is_empty())
    {
        println!("\nNo open positions.");
    }

    Ok(())
}
