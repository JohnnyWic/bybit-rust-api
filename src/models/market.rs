//! Market data models.

use rust_decimal::Decimal;
use serde::Deserialize;

/// Server time response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    /// Server time in seconds
    pub time_second: String,
    /// Server time in nanoseconds
    pub time_nano: String,
}

/// Instruments info response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfo {
    /// Category
    pub category: String,
    /// List of instruments
    pub list: Vec<InstrumentInfo>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Single instrument info.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfo {
    /// Symbol name
    pub symbol: String,
    /// Contract type
    #[serde(default)]
    pub contract_type: String,
    /// Trading status
    pub status: String,
    /// Base coin
    #[serde(default)]
    pub base_coin: String,
    /// Quote coin
    #[serde(default)]
    pub quote_coin: String,
    /// Settle coin
    #[serde(default)]
    pub settle_coin: String,
    /// Launch time
    #[serde(default)]
    pub launch_time: String,
    /// Delivery time
    #[serde(default)]
    pub delivery_time: String,
    /// Delivery fee rate
    #[serde(default)]
    pub delivery_fee_rate: String,
    /// Price scale
    #[serde(default)]
    pub price_scale: String,
    /// Leverage filter
    #[serde(default)]
    pub leverage_filter: Option<LeverageFilter>,
    /// Price filter
    #[serde(default)]
    pub price_filter: Option<PriceFilter>,
    /// Lot size filter
    #[serde(default)]
    pub lot_size_filter: Option<LotSizeFilter>,
}

/// Leverage filter.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    /// Min leverage
    pub min_leverage: String,
    /// Max leverage
    pub max_leverage: String,
    /// Leverage step
    pub leverage_step: String,
}

/// Price filter.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    /// Min price
    pub min_price: String,
    /// Max price
    pub max_price: String,
    /// Tick size
    pub tick_size: String,
}

/// Lot size filter.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    /// Max order qty
    #[serde(default)]
    pub max_order_qty: String,
    /// Min order qty
    #[serde(default)]
    pub min_order_qty: String,
    /// Qty step
    #[serde(default)]
    pub qty_step: String,
    /// Post only max order qty
    #[serde(default)]
    pub post_only_max_order_qty: String,
    /// Base precision
    #[serde(default)]
    pub base_precision: String,
    /// Quote precision
    #[serde(default)]
    pub quote_precision: String,
    /// Min order amt
    #[serde(default)]
    pub min_order_amt: String,
    /// Max order amt
    #[serde(default)]
    pub max_order_amt: String,
}

/// Orderbook response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    /// Symbol
    pub s: String,
    /// Bids [price, size]
    pub b: Vec<[String; 2]>,
    /// Asks [price, size]
    pub a: Vec<[String; 2]>,
    /// Timestamp
    pub ts: u64,
    /// Update ID
    pub u: u64,
}

/// Tickers response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tickers {
    /// Category
    pub category: String,
    /// List of tickers
    pub list: Vec<Ticker>,
}

/// Single ticker.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    /// Symbol
    pub symbol: String,
    /// Last price
    #[serde(default)]
    pub last_price: String,
    /// Index price
    #[serde(default)]
    pub index_price: String,
    /// Mark price
    #[serde(default)]
    pub mark_price: String,
    /// Previous 24h price
    #[serde(default)]
    pub prev_price_24h: String,
    /// Price change 24h percentage
    #[serde(default)]
    pub price_24h_pcnt: String,
    /// High price 24h
    #[serde(default)]
    pub high_price_24h: String,
    /// Low price 24h
    #[serde(default)]
    pub low_price_24h: String,
    /// Previous 1h price
    #[serde(default)]
    pub prev_price_1h: String,
    /// Open interest
    #[serde(default)]
    pub open_interest: String,
    /// Open interest value
    #[serde(default)]
    pub open_interest_value: String,
    /// Turnover 24h
    #[serde(default)]
    pub turnover_24h: String,
    /// Volume 24h
    #[serde(default)]
    pub volume_24h: String,
    /// Funding rate
    #[serde(default)]
    pub funding_rate: String,
    /// Next funding time
    #[serde(default)]
    pub next_funding_time: String,
    /// Bid price
    #[serde(default)]
    pub bid_1_price: String,
    /// Bid size
    #[serde(default)]
    pub bid_1_size: String,
    /// Ask price
    #[serde(default)]
    pub ask_1_price: String,
    /// Ask size
    #[serde(default)]
    pub ask_1_size: String,
}

/// Kline response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Klines {
    /// Category
    pub category: String,
    /// Symbol
    pub symbol: String,
    /// List of klines [timestamp, open, high, low, close, volume, turnover]
    pub list: Vec<Vec<String>>,
}

/// Parsed kline data.
#[derive(Debug, Clone)]
pub struct Kline {
    /// Start time in milliseconds
    pub start_time: u64,
    /// Open price
    pub open: Decimal,
    /// High price
    pub high: Decimal,
    /// Low price
    pub low: Decimal,
    /// Close price
    pub close: Decimal,
    /// Volume
    pub volume: Decimal,
    /// Turnover
    pub turnover: Decimal,
}

/// Funding rate history response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingHistory {
    /// Category
    pub category: String,
    /// List of funding records
    pub list: Vec<FundingRecord>,
}

/// Single funding record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRecord {
    /// Symbol
    pub symbol: String,
    /// Funding rate
    pub funding_rate: String,
    /// Funding rate timestamp
    pub funding_rate_timestamp: String,
}

/// Recent trades response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrades {
    /// Category
    pub category: String,
    /// List of trades
    pub list: Vec<Trade>,
}

/// Single trade.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    /// Exec ID
    pub exec_id: String,
    /// Symbol
    pub symbol: String,
    /// Price
    pub price: String,
    /// Size
    pub size: String,
    /// Side
    pub side: String,
    /// Time
    pub time: String,
    /// Is block trade
    #[serde(default)]
    pub is_block_trade: bool,
}

/// Open interest response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    /// Category
    pub category: String,
    /// Symbol
    pub symbol: String,
    /// List of open interest data
    pub list: Vec<OpenInterestRecord>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Single open interest record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestRecord {
    /// Open interest
    pub open_interest: String,
    /// Timestamp
    pub timestamp: String,
}

/// Risk limit response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimits {
    /// Category
    pub category: String,
    /// List of risk limits
    pub list: Vec<RiskLimit>,
}

/// Single risk limit.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimit {
    /// ID
    pub id: i32,
    /// Symbol
    pub symbol: String,
    /// Risk limit value
    pub risk_limit_value: String,
    /// Maintenance margin
    pub maintenance_margin: String,
    /// Initial margin
    pub initial_margin: String,
    /// Max leverage
    pub max_leverage: String,
}
