//! Market data API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::*;

impl BybitClient {
    /// Get server time.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use bybit_api::BybitClient;
    /// # async fn example() -> bybit_api::Result<()> {
    /// let client = BybitClient::testnet("key", "secret")?;
    /// let time = client.get_server_time().await?;
    /// println!("Server time: {}", time.time_second);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        self.get_public("/v5/market/time", &[]).await
    }

    /// Get instruments info.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    ///
    /// # Example
    /// ```rust,no_run
    /// # use bybit_api::{BybitClient, Category};
    /// # async fn example() -> bybit_api::Result<()> {
    /// let client = BybitClient::testnet("key", "secret")?;
    /// let info = client.get_instruments_info(Category::Linear, Some("BTCUSDT")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_instruments_info(
        &self,
        category: Category,
        symbol: Option<&str>,
    ) -> Result<InstrumentsInfo> {
        let mut params = vec![("category", category.to_string())];

        let symbol_str;
        if let Some(s) = symbol {
            symbol_str = s.to_string();
            params.push(("symbol", symbol_str.clone()));
        }

        let params_ref: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.get_public("/v5/market/instruments-info", &params_ref)
            .await
    }

    /// Get orderbook.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `limit` - Optional depth limit (1-500, default 25)
    pub async fn get_orderbook(
        &self,
        category: Category,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<Orderbook> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(25).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/orderbook", &params).await
    }

    /// Get tickers.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter (returns all if None)
    pub async fn get_tickers(&self, category: Category, symbol: Option<&str>) -> Result<Tickers> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get_public("/v5/market/tickers", &params).await
    }

    /// Get klines (candlestick data).
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `interval` - Kline interval
    /// * `limit` - Optional limit (1-1000, default 200)
    pub async fn get_klines(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        limit: Option<u32>,
    ) -> Result<Klines> {
        let cat_str = category.to_string();
        let interval_str = interval.to_string();
        let limit_str = limit.unwrap_or(200).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("interval", interval_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/kline", &params).await
    }

    /// Get funding rate history.
    ///
    /// # Arguments
    /// * `category` - Product category (linear or inverse)
    /// * `symbol` - Symbol name
    /// * `limit` - Optional limit (default 200)
    pub async fn get_funding_history(
        &self,
        category: Category,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<FundingHistory> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(200).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/funding/history", &params).await
    }

    /// Get recent trades.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `limit` - Optional limit (1-1000, default 500)
    pub async fn get_recent_trades(
        &self,
        category: Category,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<RecentTrades> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(500).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/recent-trade", &params).await
    }

    /// Get open interest.
    ///
    /// # Arguments
    /// * `category` - Product category (linear or inverse)
    /// * `symbol` - Symbol name
    /// * `interval_time` - Interval (5min, 15min, 30min, 1h, 4h, 1d)
    /// * `limit` - Optional limit (default 50)
    pub async fn get_open_interest(
        &self,
        category: Category,
        symbol: &str,
        interval_time: &str,
        limit: Option<u32>,
    ) -> Result<OpenInterest> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(50).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("intervalTime", interval_time),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/open-interest", &params).await
    }

    /// Get risk limit info.
    ///
    /// # Arguments
    /// * `category` - Product category (linear or inverse)
    /// * `symbol` - Optional symbol filter
    pub async fn get_risk_limit(
        &self,
        category: Category,
        symbol: Option<&str>,
    ) -> Result<RiskLimits> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get_public("/v5/market/risk-limit", &params).await
    }

    /// Get mark price kline.
    pub async fn get_mark_price_kline(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        limit: Option<u32>,
    ) -> Result<Klines> {
        let cat_str = category.to_string();
        let interval_str = interval.to_string();
        let limit_str = limit.unwrap_or(200).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("interval", interval_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/mark-price-kline", &params)
            .await
    }

    /// Get index price kline.
    pub async fn get_index_price_kline(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        limit: Option<u32>,
    ) -> Result<Klines> {
        let cat_str = category.to_string();
        let interval_str = interval.to_string();
        let limit_str = limit.unwrap_or(200).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("interval", interval_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/index-price-kline", &params)
            .await
    }

    /// Get premium index price kline.
    pub async fn get_premium_index_price_kline(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        limit: Option<u32>,
    ) -> Result<Klines> {
        let cat_str = category.to_string();
        let interval_str = interval.to_string();
        let limit_str = limit.unwrap_or(200).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("interval", interval_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/premium-index-price-kline", &params)
            .await
    }
}
