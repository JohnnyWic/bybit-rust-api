//! Position API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::position::*;
use crate::models::*;

impl BybitClient {
    /// Get position list.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    /// * `settle_coin` - Optional settle coin filter (e.g., "USDT")
    pub async fn get_positions(
        &self,
        category: Category,
        symbol: Option<&str>,
        settle_coin: Option<&str>,
    ) -> Result<PositionList> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }
        if let Some(sc) = settle_coin {
            params.push(("settleCoin", sc));
        }

        self.get("/v5/position/list", &params).await
    }

    /// Set leverage.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `buy_leverage` - Buy leverage
    /// * `sell_leverage` - Sell leverage
    pub async fn set_leverage(
        &self,
        category: Category,
        symbol: &str,
        buy_leverage: &str,
        sell_leverage: &str,
    ) -> Result<serde_json::Value> {
        let params = SetLeverageParams {
            category,
            symbol: symbol.to_string(),
            buy_leverage: buy_leverage.to_string(),
            sell_leverage: sell_leverage.to_string(),
        };

        self.post("/v5/position/set-leverage", &params).await
    }

    /// Set trading stop (TP/SL).
    ///
    /// # Arguments
    /// * `params` - Trading stop parameters
    pub async fn set_trading_stop(&self, params: TradingStopParams) -> Result<serde_json::Value> {
        self.post("/v5/position/trading-stop", &params).await
    }

    /// Switch position mode.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `mode` - Position mode (0=merged, 3=both sides)
    pub async fn switch_position_mode(
        &self,
        category: Category,
        mode: PositionMode,
    ) -> Result<serde_json::Value> {
        let params = SwitchPositionModeParams {
            category,
            symbol: None,
            coin: None,
            mode: mode as i32,
        };

        self.post("/v5/position/switch-mode", &params).await
    }

    /// Set risk limit.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `risk_id` - Risk limit ID
    pub async fn set_risk_limit(
        &self,
        category: Category,
        symbol: &str,
        risk_id: i32,
    ) -> Result<serde_json::Value> {
        let params = SetRiskLimitParams {
            category,
            symbol: symbol.to_string(),
            risk_id,
            position_idx: None,
        };

        self.post("/v5/position/set-risk-limit", &params).await
    }

    /// Add or reduce margin.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `margin` - Margin amount (positive to add, negative to reduce)
    pub async fn add_margin(
        &self,
        category: Category,
        symbol: &str,
        margin: &str,
    ) -> Result<serde_json::Value> {
        let params = AddMarginParams {
            category,
            symbol: symbol.to_string(),
            margin: margin.to_string(),
            position_idx: None,
        };

        self.post("/v5/position/add-margin", &params).await
    }

    /// Get closed PnL history.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    /// * `limit` - Optional limit (default 20)
    pub async fn get_closed_pnl(
        &self,
        category: Category,
        symbol: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ClosedPnlList> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(20).to_string();
        let mut params = vec![
            ("category", cat_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get("/v5/position/closed-pnl", &params).await
    }

    /// Get execution list (trade history).
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    /// * `limit` - Optional limit (default 50)
    pub async fn get_executions(
        &self,
        category: Category,
        symbol: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ExecutionList> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(50).to_string();
        let mut params = vec![
            ("category", cat_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get("/v5/execution/list", &params).await
    }
}
