//! Asset API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::asset::*;
use tracing::info;
use uuid::Uuid;

impl BybitClient {
    /// Get coin info.
    ///
    /// # Arguments
    /// * `coin` - Optional coin filter
    pub async fn get_coin_info(&self, coin: Option<&str>) -> Result<CoinInfoResponse> {
        let mut params = vec![];

        if let Some(c) = coin {
            params.push(("coin", c));
        }

        self.get("/v5/asset/coin/query-info", &params).await
    }

    /// Internal transfer between accounts.
    ///
    /// # Arguments
    /// * `coin` - Coin to transfer
    /// * `amount` - Amount to transfer
    /// * `from_account` - Source account type
    /// * `to_account` - Destination account type
    pub async fn internal_transfer(
        &self,
        coin: &str,
        amount: &str,
        from_account: &str,
        to_account: &str,
    ) -> Result<TransferResponse> {
        let params = InternalTransferParams {
            transfer_id: Uuid::new_v4().to_string(),
            coin: coin.to_string(),
            amount: amount.to_string(),
            from_account_type: from_account.to_string(),
            to_account_type: to_account.to_string(),
        };

        // Validate parameters
        params.validate()?;

        info!(
            coin = %coin,
            amount = %amount,
            from = %from_account,
            to = %to_account,
            "Internal transfer"
        );

        self.post("/v5/asset/transfer/inter-transfer", &params)
            .await
    }

    /// Get internal transfer list.
    ///
    /// # Arguments
    /// * `coin` - Optional coin filter
    /// * `limit` - Optional limit (default 20)
    pub async fn get_internal_transfer_list(
        &self,
        coin: Option<&str>,
        limit: Option<u32>,
    ) -> Result<TransferList> {
        let limit_str = limit.unwrap_or(20).to_string();
        let mut params = vec![("limit", limit_str.as_str())];

        if let Some(c) = coin {
            params.push(("coin", c));
        }

        self.get("/v5/asset/transfer/query-inter-transfer-list", &params)
            .await
    }

    /// Get deposit address.
    ///
    /// # Arguments
    /// * `coin` - Coin name
    /// * `chain_type` - Optional chain type
    pub async fn get_deposit_address(
        &self,
        coin: &str,
        chain_type: Option<&str>,
    ) -> Result<DepositAddressResponse> {
        let mut params = vec![("coin", coin)];

        if let Some(ct) = chain_type {
            params.push(("chainType", ct));
        }

        self.get("/v5/asset/deposit/query-address", &params).await
    }

    /// Get deposit records.
    ///
    /// # Arguments
    /// * `coin` - Optional coin filter
    /// * `limit` - Optional limit (default 50)
    pub async fn get_deposit_records(
        &self,
        coin: Option<&str>,
        limit: Option<u32>,
    ) -> Result<DepositRecords> {
        let limit_str = limit.unwrap_or(50).to_string();
        let mut params = vec![("limit", limit_str.as_str())];

        if let Some(c) = coin {
            params.push(("coin", c));
        }

        self.get("/v5/asset/deposit/query-record", &params).await
    }

    /// Withdraw funds (REQUIRES STRICT VALIDATION).
    ///
    /// # Arguments
    /// * `params` - Withdraw parameters
    ///
    /// # Safety
    /// This function validates all parameters before sending to prevent fund loss.
    pub async fn withdraw(&self, params: WithdrawParams) -> Result<WithdrawResponse> {
        // CRITICAL: Validate all parameters (fund safety)
        params.validate()?;

        info!(
            coin = %params.coin,
            chain = %params.chain,
            address = %params.address,
            amount = %params.amount,
            "Initiating withdrawal"
        );

        self.post("/v5/asset/withdraw/create", &params).await
    }

    /// Cancel a pending withdrawal.
    ///
    /// # Arguments
    /// * `withdraw_id` - Withdraw ID to cancel
    pub async fn cancel_withdraw(&self, withdraw_id: &str) -> Result<serde_json::Value> {
        let params = CancelWithdrawParams {
            id: withdraw_id.to_string(),
        };

        info!(withdraw_id = %withdraw_id, "Cancelling withdrawal");

        self.post("/v5/asset/withdraw/cancel", &params).await
    }

    /// Get withdraw records.
    ///
    /// # Arguments
    /// * `coin` - Optional coin filter
    /// * `limit` - Optional limit (default 50)
    pub async fn get_withdraw_records(
        &self,
        coin: Option<&str>,
        limit: Option<u32>,
    ) -> Result<WithdrawRecords> {
        let limit_str = limit.unwrap_or(50).to_string();
        let mut params = vec![("limit", limit_str.as_str())];

        if let Some(c) = coin {
            params.push(("coin", c));
        }

        self.get("/v5/asset/withdraw/query-record", &params).await
    }

    /// Get withdrawable amount.
    ///
    /// # Arguments
    /// * `coin` - Coin name
    pub async fn get_withdrawable_amount(&self, coin: &str) -> Result<WithdrawableAmount> {
        let params = vec![("coin", coin)];

        self.get("/v5/asset/withdraw/withdrawable-amount", &params)
            .await
    }
}
