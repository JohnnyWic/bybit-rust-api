//! Account API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::account::*;
use crate::models::*;

impl BybitClient {
    /// Get wallet balance.
    ///
    /// # Arguments
    /// * `account_type` - Account type (UNIFIED, CONTRACT, etc.)
    pub async fn get_wallet_balance(&self, account_type: AccountType) -> Result<WalletBalance> {
        let account_type_str = format!("{:?}", account_type);
        let params = vec![("accountType", account_type_str.as_str())];

        self.get("/v5/account/wallet-balance", &params).await
    }

    /// Get account info.
    pub async fn get_account_info(&self) -> Result<AccountInfo> {
        self.get("/v5/account/info", &[]).await
    }

    /// Get fee rate.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    pub async fn get_fee_rate(&self, category: Category, symbol: Option<&str>) -> Result<FeeRates> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get("/v5/account/fee-rate", &params).await
    }

    /// Get transaction log.
    ///
    /// # Arguments
    /// * `category` - Optional category filter
    /// * `limit` - Optional limit (default 20)
    pub async fn get_transaction_log(
        &self,
        category: Option<Category>,
        limit: Option<u32>,
    ) -> Result<TransactionLogs> {
        let limit_str = limit.unwrap_or(20).to_string();
        let mut params = vec![("limit", limit_str.as_str())];

        let cat_str;
        if let Some(c) = category {
            cat_str = c.to_string();
            params.push(("category", cat_str.as_str()));
        }

        self.get("/v5/account/transaction-log", &params).await
    }

    /// Set margin mode.
    ///
    /// # Arguments
    /// * `margin_mode` - Margin mode (REGULAR_MARGIN, PORTFOLIO_MARGIN)
    pub async fn set_margin_mode(&self, margin_mode: MarginMode) -> Result<serde_json::Value> {
        let mode_str = format!("{:?}", margin_mode);
        let params = SetMarginModeParams {
            set_margin_mode: mode_str,
        };

        self.post("/v5/account/set-margin-mode", &params).await
    }

    /// Get collateral info.
    ///
    /// # Arguments
    /// * `currency` - Optional currency filter
    pub async fn get_collateral_info(&self, currency: Option<&str>) -> Result<CollateralInfo> {
        let mut params = vec![];

        if let Some(c) = currency {
            params.push(("currency", c));
        }

        self.get("/v5/account/collateral-info", &params).await
    }

    /// Get borrow history.
    ///
    /// # Arguments
    /// * `currency` - Optional currency filter
    /// * `limit` - Optional limit (default 20)
    pub async fn get_borrow_history(
        &self,
        currency: Option<&str>,
        limit: Option<u32>,
    ) -> Result<BorrowHistory> {
        let limit_str = limit.unwrap_or(20).to_string();
        let mut params = vec![("limit", limit_str.as_str())];

        if let Some(c) = currency {
            params.push(("currency", c));
        }

        self.get("/v5/account/borrow-history", &params).await
    }
}
