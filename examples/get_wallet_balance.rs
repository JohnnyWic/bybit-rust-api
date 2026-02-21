//! Example: Get wallet balance

use bybit_api::{AccountType, BybitClient};
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

    // Get unified account balance
    println!("Fetching wallet balance...");
    let balance = client.get_wallet_balance(AccountType::UNIFIED).await?;

    for account in &balance.list {
        println!("\nAccount Type: {}", account.account_type);
        println!("Total Equity: {}", account.total_equity);
        println!("Total Wallet Balance: {}", account.total_wallet_balance);
        println!(
            "Total Available Balance: {}",
            account.total_available_balance
        );

        println!("\nCoin Balances:");
        for coin in &account.coin {
            if !coin.wallet_balance.is_empty() && coin.wallet_balance != "0" {
                println!(
                    "  {} - Balance: {}, Available: {}, USD Value: {}",
                    coin.coin, coin.wallet_balance, coin.available_to_withdraw, coin.usd_value
                );
            }
        }
    }

    Ok(())
}
