# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-22

### Added

- Initial release of bybit-api Rust SDK
- **HTTP Client**
  - Support for mainnet, testnet, and demo environments
  - HMAC-SHA256 request signing
  - Timeout protection on all requests
  - Automatic error parsing and handling
- **Market API**
  - `get_server_time()` - Get server time
  - `get_instruments_info()` - Get instruments info
  - `get_orderbook()` - Get orderbook depth
  - `get_tickers()` - Get 24hr ticker
  - `get_klines()` - Get kline/candlestick data
  - `get_funding_history()` - Get funding rate history
  - `get_recent_trades()` - Get recent trades
  - `get_open_interest()` - Get open interest
  - `get_risk_limits()` - Get risk limits
- **Trade API**
  - `place_order()` - Place order with validation
  - `amend_order()` - Amend order
  - `cancel_order()` - Cancel order
  - `cancel_all_orders()` - Cancel all orders
  - `place_batch_order()` - Batch place orders
  - `amend_batch_order()` - Batch amend orders
  - `cancel_batch_order()` - Batch cancel orders
  - `get_open_orders()` - Get open orders
  - `get_order_history()` - Get order history
- **Position API**
  - `get_positions()` - Get position list
  - `set_leverage()` - Set leverage
  - `set_trading_stop()` - Set TP/SL
  - `switch_position_mode()` - Switch position mode
  - `set_risk_limit()` - Set risk limit
  - `add_margin()` - Add/reduce margin
  - `get_closed_pnl()` - Get closed PnL
  - `get_executions()` - Get execution list
- **Account API**
  - `get_wallet_balance()` - Get wallet balance
  - `get_account_info()` - Get account info
  - `get_fee_rate()` - Get fee rates
  - `get_transaction_log()` - Get transaction log
  - `set_margin_mode()` - Set margin mode
  - `get_collateral_info()` - Get collateral info
  - `get_borrow_history()` - Get borrow history
- **Asset API**
  - `get_coin_info()` - Get coin info
  - `internal_transfer()` - Internal transfer with validation
  - `get_internal_transfer_list()` - Get transfer records
  - `get_deposit_address()` - Get deposit address
  - `get_deposit_records()` - Get deposit records
  - `withdraw()` - Withdraw with strict validation (fund safety)
  - `cancel_withdraw()` - Cancel withdrawal
  - `get_withdraw_records()` - Get withdrawal records
  - `get_withdrawable_amount()` - Get withdrawable amount
- **WebSocket Client**
  - Public channel subscriptions (orderbook, trade, ticker, kline)
  - Private channel subscriptions (position, order, wallet, execution)
  - Automatic ping/pong heartbeat
  - Authentication for private channels
  - Subscription management
- **Examples**
  - `market_ticker.rs` - Market data example
  - `place_order.rs` - Order placement example
  - `get_positions.rs` - Position query example
  - `get_wallet_balance.rs` - Wallet balance example
  - `websocket_orderbook.rs` - WebSocket public example
  - `websocket_private.rs` - WebSocket private example
- **CI/CD**
  - GitHub Actions for CI (fmt, clippy, build, test)
  - GitHub Actions for release (auto publish to crates.io)

### Security

- Zero-panic strategy: No `unwrap()` or `expect()` in library code
- Parameter validation before sending orders and withdrawals
- Large withdrawal warnings for fund safety
- Timeout protection on all HTTP and WebSocket operations
