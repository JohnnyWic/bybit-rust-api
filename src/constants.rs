//! Constants for Bybit API endpoints.

// =============================================================================
// REST API Base URLs
// =============================================================================

/// Mainnet REST API
pub const MAINNET: &str = "https://api.bybit.com";

/// Mainnet alternative (bytick)
pub const MAINNET_BYTICK: &str = "https://api.bytick.com";

/// Testnet REST API
pub const TESTNET: &str = "https://api-testnet.bybit.com";

/// Demo trading environment
pub const DEMO: &str = "https://api-demo.bybit.com";

/// Netherlands
pub const NETHERLANDS: &str = "https://api.bybit.nl";

/// Hong Kong
pub const HONGKONG: &str = "https://api.byhkbit.com";

/// Turkey
pub const TURKEY: &str = "https://api.bybit-tr.com";

/// Kazakhstan
pub const KAZAKHSTAN: &str = "https://api.bybit.kz";

// =============================================================================
// WebSocket Public Channels - Mainnet
// =============================================================================

/// Spot public channel (mainnet)
pub const MAINNET_WS_PUBLIC_SPOT: &str = "wss://stream.bybit.com/v5/public/spot";

/// Linear public channel (mainnet)
pub const MAINNET_WS_PUBLIC_LINEAR: &str = "wss://stream.bybit.com/v5/public/linear";

/// Inverse public channel (mainnet)
pub const MAINNET_WS_PUBLIC_INVERSE: &str = "wss://stream.bybit.com/v5/public/inverse";

/// Option public channel (mainnet)
pub const MAINNET_WS_PUBLIC_OPTION: &str = "wss://stream.bybit.com/v5/public/option";

/// Spread public channel (mainnet)
pub const MAINNET_WS_PUBLIC_SPREAD: &str = "wss://stream.bybit.com/v5/public/spread";

// =============================================================================
// WebSocket Public Channels - Testnet
// =============================================================================

/// Spot public channel (testnet)
pub const TESTNET_WS_PUBLIC_SPOT: &str = "wss://stream-testnet.bybit.com/v5/public/spot";

/// Linear public channel (testnet)
pub const TESTNET_WS_PUBLIC_LINEAR: &str = "wss://stream-testnet.bybit.com/v5/public/linear";

/// Inverse public channel (testnet)
pub const TESTNET_WS_PUBLIC_INVERSE: &str = "wss://stream-testnet.bybit.com/v5/public/inverse";

/// Option public channel (testnet)
pub const TESTNET_WS_PUBLIC_OPTION: &str = "wss://stream-testnet.bybit.com/v5/public/option";

/// Spread public channel (testnet)
pub const TESTNET_WS_PUBLIC_SPREAD: &str = "wss://stream-testnet.bybit.com/v5/public/spread";

// =============================================================================
// WebSocket Private Channels
// =============================================================================

/// Private channel (mainnet)
pub const MAINNET_WS_PRIVATE: &str = "wss://stream.bybit.com/v5/private";

/// Trade channel (mainnet)
pub const MAINNET_WS_TRADE: &str = "wss://stream.bybit.com/v5/trade";

/// Private channel (testnet)
pub const TESTNET_WS_PRIVATE: &str = "wss://stream-testnet.bybit.com/v5/private";

/// Trade channel (testnet)
pub const TESTNET_WS_TRADE: &str = "wss://stream-testnet.bybit.com/v5/trade";

/// Private channel (demo)
pub const DEMO_WS_PRIVATE: &str = "wss://stream-demo.bybit.com/v5/private";

/// Trade channel (demo)
pub const DEMO_WS_TRADE: &str = "wss://stream-demo.bybit.com/v5/trade";

// =============================================================================
// HTTP Headers
// =============================================================================

/// API key header
pub const HEADER_API_KEY: &str = "X-BAPI-API-KEY";

/// Signature header
pub const HEADER_SIGN: &str = "X-BAPI-SIGN";

/// Sign type header
pub const HEADER_SIGN_TYPE: &str = "X-BAPI-SIGN-TYPE";

/// Timestamp header
pub const HEADER_TIMESTAMP: &str = "X-BAPI-TIMESTAMP";

/// Receive window header
pub const HEADER_RECV_WINDOW: &str = "X-BAPI-RECV-WINDOW";

// =============================================================================
// Default Values
// =============================================================================

/// Default receive window in milliseconds
pub const DEFAULT_RECV_WINDOW: u64 = 5000;

/// Default request timeout in seconds
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default WebSocket ping interval in seconds
pub const DEFAULT_WS_PING_INTERVAL: u64 = 20;
