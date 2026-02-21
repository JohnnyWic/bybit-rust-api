//! Example: Get market tickers

use bybit_api::{BybitClient, Category};

#[tokio::main]
async fn main() -> bybit_api::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create client for testnet (no API key needed for public endpoints)
    let client = BybitClient::testnet("", "")?;

    // Get all linear tickers
    println!("Fetching linear tickers...");
    let tickers = client.get_tickers(Category::Linear, None).await?;

    println!("Found {} tickers", tickers.list.len());

    // Print first 5 tickers
    for ticker in tickers.list.iter().take(5) {
        println!(
            "  {} - Last: {}, 24h Change: {}%",
            ticker.symbol, ticker.last_price, ticker.price_24h_pcnt
        );
    }

    // Get specific ticker
    println!("\nFetching BTCUSDT ticker...");
    let btc_ticker = client
        .get_tickers(Category::Linear, Some("BTCUSDT"))
        .await?;

    if let Some(btc) = btc_ticker.list.first() {
        println!("BTCUSDT:");
        println!("  Last Price: {}", btc.last_price);
        println!("  Mark Price: {}", btc.mark_price);
        println!("  Index Price: {}", btc.index_price);
        println!("  24h High: {}", btc.high_price_24h);
        println!("  24h Low: {}", btc.low_price_24h);
        println!("  24h Volume: {}", btc.volume_24h);
        println!("  Funding Rate: {}", btc.funding_rate);
    }

    Ok(())
}
