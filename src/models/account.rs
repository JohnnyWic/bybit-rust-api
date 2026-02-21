//! Account models.

use serde::{Deserialize, Serialize};

/// Wallet balance response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    /// List of account balances
    pub list: Vec<AccountBalance>,
}

/// Account balance.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    /// Account type
    pub account_type: String,
    /// Account LTV
    #[serde(default)]
    pub account_l_t_v: String,
    /// Account IM rate
    #[serde(default)]
    pub account_i_m_rate: String,
    /// Account MM rate
    #[serde(default)]
    pub account_m_m_rate: String,
    /// Total equity
    #[serde(default)]
    pub total_equity: String,
    /// Total wallet balance
    #[serde(default)]
    pub total_wallet_balance: String,
    /// Total margin balance
    #[serde(default)]
    pub total_margin_balance: String,
    /// Total available balance
    #[serde(default)]
    pub total_available_balance: String,
    /// Total perp UPL
    #[serde(default)]
    pub total_perp_u_p_l: String,
    /// Total initial margin
    #[serde(default)]
    pub total_initial_margin: String,
    /// Total maintenance margin
    #[serde(default)]
    pub total_maintenance_margin: String,
    /// Coin list
    #[serde(default)]
    pub coin: Vec<CoinBalance>,
}

/// Coin balance.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinBalance {
    /// Coin name
    pub coin: String,
    /// Equity
    #[serde(default)]
    pub equity: String,
    /// USD value
    #[serde(default)]
    pub usd_value: String,
    /// Wallet balance
    #[serde(default)]
    pub wallet_balance: String,
    /// Free amount
    #[serde(default)]
    pub free: String,
    /// Locked amount
    #[serde(default)]
    pub locked: String,
    /// Available to withdraw
    #[serde(default)]
    pub available_to_withdraw: String,
    /// Available to borrow
    #[serde(default)]
    pub available_to_borrow: String,
    /// Borrow amount
    #[serde(default)]
    pub borrow_amount: String,
    /// Accrued interest
    #[serde(default)]
    pub accrued_interest: String,
    /// Total order IM
    #[serde(default)]
    pub total_order_i_m: String,
    /// Total position IM
    #[serde(default)]
    pub total_position_i_m: String,
    /// Total position MM
    #[serde(default)]
    pub total_position_m_m: String,
    /// Unrealised PnL
    #[serde(default)]
    pub unrealised_pnl: String,
    /// Cumulative realised PnL
    #[serde(default)]
    pub cum_realised_pnl: String,
}

/// Account info response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// Unified margin status
    #[serde(default)]
    pub unified_margin_status: i32,
    /// Margin mode
    #[serde(default)]
    pub margin_mode: String,
    /// DCP status
    #[serde(default)]
    pub dcp_status: String,
    /// Time window
    #[serde(default)]
    pub time_window: i32,
    /// SMP group
    #[serde(default)]
    pub smp_group: i32,
    /// Is master trader
    #[serde(default)]
    pub is_master_trader: bool,
    /// Spot hedging status
    #[serde(default)]
    pub spot_hedging_status: String,
    /// Updated time
    #[serde(default)]
    pub updated_time: String,
}

/// Fee rate response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRates {
    /// Category
    pub category: String,
    /// List of fee rates
    pub list: Vec<FeeRate>,
}

/// Fee rate.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRate {
    /// Symbol
    pub symbol: String,
    /// Base coin
    #[serde(default)]
    pub base_coin: String,
    /// Taker fee rate
    pub taker_fee_rate: String,
    /// Maker fee rate
    pub maker_fee_rate: String,
}

/// Transaction log response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogs {
    /// List of transactions
    pub list: Vec<TransactionLog>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Transaction log.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLog {
    /// ID
    pub id: String,
    /// Symbol
    #[serde(default)]
    pub symbol: String,
    /// Category
    pub category: String,
    /// Side
    #[serde(default)]
    pub side: String,
    /// Transaction time
    pub transaction_time: String,
    /// Type
    #[serde(rename = "type")]
    pub tx_type: String,
    /// Qty
    #[serde(default)]
    pub qty: String,
    /// Size
    #[serde(default)]
    pub size: String,
    /// Currency
    pub currency: String,
    /// Trade price
    #[serde(default)]
    pub trade_price: String,
    /// Funding
    #[serde(default)]
    pub funding: String,
    /// Fee
    #[serde(default)]
    pub fee: String,
    /// Cash flow
    #[serde(default)]
    pub cash_flow: String,
    /// Change
    pub change: String,
    /// Cash balance
    pub cash_balance: String,
}

/// Set margin mode request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeParams {
    /// Set margin mode
    pub set_margin_mode: String,
}

/// Collateral info response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfo {
    /// List of collateral info
    pub list: Vec<Collateral>,
}

/// Collateral.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collateral {
    /// Currency
    pub currency: String,
    /// Hourly borrow rate
    #[serde(default)]
    pub hourly_borrow_rate: String,
    /// Max borrow amount
    #[serde(default)]
    pub max_borrowing_amount: String,
    /// Free borrow amount
    #[serde(default)]
    pub free_borrowing_amount: String,
    /// Free borrow limit
    #[serde(default)]
    pub free_borrow_limit: String,
    /// Borrow usable switch
    #[serde(default)]
    pub borrow_usable_switch: bool,
    /// Collateral switch
    #[serde(default)]
    pub collateral_switch: bool,
    /// Collateral ratio
    #[serde(default)]
    pub collateral_ratio: String,
}

/// Borrow history response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistory {
    /// List of borrow records
    pub list: Vec<BorrowRecord>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Borrow record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowRecord {
    /// Currency
    pub currency: String,
    /// Created time
    pub created_time: String,
    /// Borrow cost
    #[serde(default)]
    pub borrow_cost: String,
    /// Hourly borrow rate
    #[serde(default)]
    pub hourly_borrow_rate: String,
    /// Interest bearing borrow size
    #[serde(default)]
    pub interest_bearing_borrow_size: String,
    /// Cost exemption
    #[serde(default)]
    pub cost_exemption: String,
}
