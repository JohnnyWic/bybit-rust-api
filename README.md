# bybit-api

[![Crates.io](https://img.shields.io/crates/v/bybit-api.svg)](https://crates.io/crates/bybit-api)
[![Documentation](https://docs.rs/bybit-api/badge.svg)](https://docs.rs/bybit-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A production-grade Rust SDK for Bybit V5 API.

## Features

- **Async-first**: Built on tokio for high-performance async I/O
- **Type-safe**: Strongly typed request/response models with serde
- **Zero-panic**: No `unwrap()` or `expect()` in library code
- **Fund safety**: Parameter validation before sending orders/withdrawals
- **Complete API coverage**: REST API + WebSocket support
- **Production ready**: Timeout protection, error handling, reconnection

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bybit-api = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

### REST API

```rust
use bybit_api::{BybitClient, Category};

#[tokio::main]
async fn main() -> bybit_api::Result<()> {
    // Create client for testnet
    let client = BybitClient::testnet("your_api_key", "your_api_secret")?;

    // Get tickers (public endpoint)
    let tickers = client.get_tickers(Category::Linear, Some("BTCUSDT")).await?;
    println!("BTC Price: {}", tickers.list[0].last_price);

    // Get wallet balance (private endpoint)
    let balance = client.get_wallet_balance("UNIFIED").await?;
    println!("Total Equity: {}", balance.list[0].total_equity);

    Ok(())
}
```

### Place Order

```rust
use bybit_api::{BybitClient, Category, Side};
use bybit_api::trade::PlaceOrderParams;

#[tokio::main]
async fn main() -> bybit_api::Result<()> {
    let client = BybitClient::testnet("your_api_key", "your_api_secret")?;

    // Place a market order
    let params = PlaceOrderParams::market(
        Category::Linear,
        "BTCUSDT",
        Side::Buy,
        "0.001",
    );

    let result = client.place_order(params).await?;
    println!("Order ID: {}", result.order_id);

    Ok(())
}
```

### WebSocket

```rust
use bybit_api::websocket::BybitWebSocket;
use bybit_api::TESTNET_WS_PUBLIC_LINEAR;

#[tokio::main]
async fn main() -> bybit_api::Result<()> {
    // Create public WebSocket client
    let mut ws = BybitWebSocket::public(TESTNET_WS_PUBLIC_LINEAR);
    ws.connect().await?;

    // Subscribe to orderbook
    ws.subscribe(vec!["orderbook.50.BTCUSDT".to_string()], |msg| {
        println!("Received: {:?}", msg.topic);
    }).await?;

    // Keep connection alive
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    ws.disconnect().await?;

    Ok(())
}
```

## Environments

```rust
// Mainnet (production)
let client = BybitClient::with_credentials("key", "secret")?;

// Testnet
let client = BybitClient::testnet("key", "secret")?;

// Demo trading
let client = BybitClient::demo("key", "secret")?;
```

## API Coverage

### Market Data
- `get_server_time()` - Server time
- `get_instruments_info()` - Instruments info
- `get_orderbook()` - Orderbook depth
- `get_tickers()` - 24hr ticker
- `get_klines()` - Kline/candlestick data
- `get_funding_history()` - Funding rate history
- `get_recent_trades()` - Recent trades
- `get_open_interest()` - Open interest

### Trading
- `place_order()` - Place order
- `amend_order()` - Amend order
- `cancel_order()` - Cancel order
- `cancel_all_orders()` - Cancel all orders
- `place_batch_order()` - Batch place orders
- `get_open_orders()` - Get open orders
- `get_order_history()` - Order history

### Position
- `get_positions()` - Get positions
- `set_leverage()` - Set leverage
- `set_trading_stop()` - Set TP/SL
- `get_closed_pnl()` - Closed PnL
- `get_executions()` - Execution list

### Account
- `get_wallet_balance()` - Wallet balance
- `get_account_info()` - Account info
- `get_fee_rate()` - Fee rates
- `get_transaction_log()` - Transaction log

### Asset
- `get_coin_info()` - Coin info
- `internal_transfer()` - Internal transfer
- `get_deposit_address()` - Deposit address
- `withdraw()` - Withdraw (with validation)

### WebSocket Topics
- Public: `orderbook`, `trade`, `ticker`, `kline`
- Private: `position`, `order`, `wallet`, `execution`

## Error Handling

```rust
use bybit_api::{BybitClient, BybitError};

match client.place_order(params).await {
    Ok(result) => println!("Order placed: {}", result.order_id),
    Err(BybitError::Api { code, msg }) => {
        println!("API error {}: {}", code, msg);
        if code == 10006 {
            println!("Rate limited, please slow down");
        }
    }
    Err(BybitError::Timeout) => println!("Request timed out"),
    Err(e) => println!("Other error: {}", e),
}
```

## Examples

See the [examples](examples/) directory for more:

- `market_ticker.rs` - Fetch market data
- `place_order.rs` - Place orders
- `get_positions.rs` - Get positions
- `get_wallet_balance.rs` - Get wallet balance
- `websocket_orderbook.rs` - Subscribe to orderbook
- `websocket_private.rs` - Private channel subscriptions

Run examples:
```bash
export BYBIT_API_KEY=your_key
export BYBIT_API_SECRET=your_secret
cargo run --example market_ticker
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Links

- [Bybit API Documentation](https://bybit-exchange.github.io/docs/v5/intro)
- [crates.io](https://crates.io/crates/bybit-api)
- [docs.rs](https://docs.rs/bybit-api)
