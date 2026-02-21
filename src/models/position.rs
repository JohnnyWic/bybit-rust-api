//! Position models.

use crate::models::common::*;
use serde::{Deserialize, Serialize};

/// Position list response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionList {
    /// Category
    pub category: String,
    /// List of positions
    pub list: Vec<Position>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Position info.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Position index
    pub position_idx: i32,
    /// Risk ID
    #[serde(default)]
    pub risk_id: i32,
    /// Risk limit value
    #[serde(default)]
    pub risk_limit_value: String,
    /// Symbol
    pub symbol: String,
    /// Side
    pub side: String,
    /// Size
    pub size: String,
    /// Average entry price
    #[serde(default)]
    pub avg_price: String,
    /// Position value
    #[serde(default)]
    pub position_value: String,
    /// Trade mode (0=cross, 1=isolated)
    #[serde(default)]
    pub trade_mode: i32,
    /// Position status
    #[serde(default)]
    pub position_status: String,
    /// Leverage
    #[serde(default)]
    pub leverage: String,
    /// Mark price
    #[serde(default)]
    pub mark_price: String,
    /// Liquidation price
    #[serde(default)]
    pub liq_price: String,
    /// Bust price
    #[serde(default)]
    pub bust_price: String,
    /// Position margin
    #[serde(default)]
    pub position_mm: String,
    /// Position initial margin
    #[serde(default)]
    pub position_im: String,
    /// Take profit price
    #[serde(default)]
    pub take_profit: String,
    /// Stop loss price
    #[serde(default)]
    pub stop_loss: String,
    /// Trailing stop
    #[serde(default)]
    pub trailing_stop: String,
    /// Unrealised PnL
    #[serde(default)]
    pub unrealised_pnl: String,
    /// Cumulative realised PnL
    #[serde(default)]
    pub cum_realised_pnl: String,
    /// Created time
    #[serde(default)]
    pub created_time: String,
    /// Updated time
    #[serde(default)]
    pub updated_time: String,
}

/// Set leverage request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageParams {
    /// Category
    pub category: Category,
    /// Symbol
    pub symbol: String,
    /// Buy leverage
    pub buy_leverage: String,
    /// Sell leverage
    pub sell_leverage: String,
}

/// Trading stop request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingStopParams {
    /// Category
    pub category: Category,
    /// Symbol
    pub symbol: String,
    /// Take profit price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// Stop loss price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// Trailing stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_stop: Option<String>,
    /// Take profit trigger
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// Stop loss trigger
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Position index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
}

/// Switch position mode request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPositionModeParams {
    /// Category
    pub category: Category,
    /// Symbol (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Coin (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Mode (0=merged, 3=both sides)
    pub mode: i32,
}

/// Set risk limit request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitParams {
    /// Category
    pub category: Category,
    /// Symbol
    pub symbol: String,
    /// Risk ID
    pub risk_id: i32,
    /// Position index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
}

/// Add margin request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarginParams {
    /// Category
    pub category: Category,
    /// Symbol
    pub symbol: String,
    /// Margin amount
    pub margin: String,
    /// Position index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
}

/// Closed PnL response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlList {
    /// Category
    pub category: String,
    /// List of closed PnL records
    pub list: Vec<ClosedPnl>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Closed PnL record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnl {
    /// Symbol
    pub symbol: String,
    /// Order ID
    pub order_id: String,
    /// Side
    pub side: String,
    /// Qty
    pub qty: String,
    /// Order price
    pub order_price: String,
    /// Order type
    pub order_type: String,
    /// Exec type
    pub exec_type: String,
    /// Closed size
    pub closed_size: String,
    /// Cumulative entry value
    pub cum_entry_value: String,
    /// Average entry price
    pub avg_entry_price: String,
    /// Cumulative exit value
    pub cum_exit_value: String,
    /// Average exit price
    pub avg_exit_price: String,
    /// Closed PnL
    pub closed_pnl: String,
    /// Fill count
    pub fill_count: String,
    /// Leverage
    pub leverage: String,
    /// Created time
    pub created_time: String,
    /// Updated time
    pub updated_time: String,
}

/// Execution list response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionList {
    /// Category
    pub category: String,
    /// List of executions
    pub list: Vec<Execution>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Execution record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    /// Symbol
    pub symbol: String,
    /// Order ID
    pub order_id: String,
    /// Order link ID
    #[serde(default)]
    pub order_link_id: String,
    /// Side
    pub side: String,
    /// Order price
    pub order_price: String,
    /// Order qty
    pub order_qty: String,
    /// Order type
    pub order_type: String,
    /// Exec ID
    pub exec_id: String,
    /// Exec price
    pub exec_price: String,
    /// Exec qty
    pub exec_qty: String,
    /// Exec fee
    pub exec_fee: String,
    /// Exec type
    pub exec_type: String,
    /// Exec value
    pub exec_value: String,
    /// Fee rate
    #[serde(default)]
    pub fee_rate: String,
    /// Exec time
    pub exec_time: String,
}
