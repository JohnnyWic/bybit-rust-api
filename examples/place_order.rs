//! Example: Place an order
//!
//! NOTE: This example requires valid API credentials and will place a real order on testnet!

use bybit_api::trade::PlaceOrderParams;
use bybit_api::{BybitClient, Category, Side};
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

    // Create a market order
    let params = PlaceOrderParams::market(
        Category::Linear,
        "BTCUSDT",
        Side::Buy,
        "0.001", // Small qty for testing
    );

    println!("Placing market order...");
    println!("  Symbol: BTCUSDT");
    println!("  Side: Buy");
    println!("  Qty: 0.001");

    match client.place_order(params).await {
        Ok(result) => {
            println!("\nOrder placed successfully!");
            println!("  Order ID: {}", result.order_id);
            println!("  Order Link ID: {}", result.order_link_id);
        }
        Err(e) => {
            println!("\nFailed to place order: {}", e);
        }
    }

    // Get open orders
    println!("\nFetching open orders...");
    let orders = client
        .get_open_orders(Category::Linear, Some("BTCUSDT"))
        .await?;
    println!("Found {} open orders", orders.list.len());

    for order in &orders.list {
        println!(
            "  {} {} {} @ {} (status: {})",
            order.side, order.qty, order.symbol, order.price, order.order_status
        );
    }

    Ok(())
}
