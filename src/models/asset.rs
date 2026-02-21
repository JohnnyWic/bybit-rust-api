//! Asset models.

use crate::error::{BybitError, Result};
use serde::{Deserialize, Serialize};
use tracing::warn;

/// Coin info response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfoResponse {
    /// List of coins
    pub rows: Vec<CoinInfo>,
}

/// Coin info.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    /// Coin name
    pub coin: String,
    /// Full name
    #[serde(default)]
    pub name: String,
    /// Remain amount
    #[serde(default)]
    pub remain_amount: String,
    /// Chain list
    #[serde(default)]
    pub chains: Vec<ChainInfo>,
}

/// Chain info.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    /// Chain
    pub chain: String,
    /// Chain type
    #[serde(default)]
    pub chain_type: String,
    /// Confirmation
    #[serde(default)]
    pub confirmation: String,
    /// Withdraw fee
    #[serde(default)]
    pub withdraw_fee: String,
    /// Deposit min
    #[serde(default)]
    pub deposit_min: String,
    /// Withdraw min
    #[serde(default)]
    pub withdraw_min: String,
    /// Chain deposit enabled
    #[serde(default)]
    pub chain_deposit: String,
    /// Chain withdraw enabled
    #[serde(default)]
    pub chain_withdraw: String,
}

/// Internal transfer request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferParams {
    /// Transfer ID (UUID)
    pub transfer_id: String,
    /// Coin
    pub coin: String,
    /// Amount
    pub amount: String,
    /// From account type
    pub from_account_type: String,
    /// To account type
    pub to_account_type: String,
}

impl InternalTransferParams {
    /// Validate transfer parameters.
    pub fn validate(&self) -> Result<()> {
        if self.coin.is_empty() {
            return Err(BybitError::InvalidParam("coin cannot be empty".into()));
        }

        let amount: rust_decimal::Decimal = self
            .amount
            .parse()
            .map_err(|_| BybitError::InvalidParam("amount must be a valid number".into()))?;
        if amount <= rust_decimal::Decimal::ZERO {
            return Err(BybitError::InvalidParam("amount must be positive".into()));
        }

        Ok(())
    }
}

/// Transfer response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResponse {
    /// Transfer ID
    pub transfer_id: String,
}

/// Transfer list response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferList {
    /// List of transfers
    pub list: Vec<TransferRecord>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Transfer record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRecord {
    /// Transfer ID
    pub transfer_id: String,
    /// Coin
    pub coin: String,
    /// Amount
    pub amount: String,
    /// From account type
    pub from_account_type: String,
    /// To account type
    pub to_account_type: String,
    /// Timestamp
    pub timestamp: String,
    /// Status
    pub status: String,
}

/// Deposit address response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddressResponse {
    /// Coin
    pub coin: String,
    /// Chains
    pub chains: Vec<DepositChainAddress>,
}

/// Deposit chain address.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositChainAddress {
    /// Chain type
    pub chain_type: String,
    /// Address deposit
    pub address_deposit: String,
    /// Tag deposit
    #[serde(default)]
    pub tag_deposit: String,
    /// Chain
    pub chain: String,
}

/// Deposit records response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecords {
    /// Rows
    pub rows: Vec<DepositRecord>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Deposit record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecord {
    /// Coin
    pub coin: String,
    /// Chain
    pub chain: String,
    /// Amount
    pub amount: String,
    /// Tx ID
    pub tx_i_d: String,
    /// Status
    pub status: i32,
    /// To address
    pub to_address: String,
    /// Tag
    #[serde(default)]
    pub tag: String,
    /// Deposit fee
    #[serde(default)]
    pub deposit_fee: String,
    /// Success at
    #[serde(default)]
    pub success_at: String,
    /// Confirmations
    #[serde(default)]
    pub confirmations: String,
    /// Tx index
    #[serde(default)]
    pub tx_index: String,
    /// Block hash
    #[serde(default)]
    pub block_hash: String,
}

/// Withdraw request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawParams {
    /// Coin
    pub coin: String,
    /// Chain
    pub chain: String,
    /// Address
    pub address: String,
    /// Tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Amount
    pub amount: String,
    /// Timestamp
    pub timestamp: u64,
    /// Force chain (for tokens on multiple chains)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_chain: Option<i32>,
    /// Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}

impl WithdrawParams {
    /// Create withdraw params.
    pub fn new(coin: &str, chain: &str, address: &str, amount: &str) -> Self {
        Self {
            coin: coin.to_string(),
            chain: chain.to_string(),
            address: address.to_string(),
            tag: None,
            amount: amount.to_string(),
            timestamp: crate::auth::get_timestamp(),
            force_chain: None,
            account_type: None,
        }
    }

    /// Validate withdraw parameters (FUND SAFETY).
    pub fn validate(&self) -> Result<()> {
        if self.coin.is_empty() {
            return Err(BybitError::InvalidParam("coin cannot be empty".into()));
        }

        if self.chain.is_empty() {
            return Err(BybitError::InvalidParam("chain cannot be empty".into()));
        }

        if self.address.is_empty() {
            return Err(BybitError::InvalidParam("address cannot be empty".into()));
        }

        let amount: rust_decimal::Decimal = self
            .amount
            .parse()
            .map_err(|_| BybitError::InvalidParam("amount must be a valid number".into()))?;
        if amount <= rust_decimal::Decimal::ZERO {
            return Err(BybitError::InvalidParam("amount must be positive".into()));
        }

        // Log warning for large withdrawals
        if amount > rust_decimal::Decimal::from(10000) {
            warn!(
                coin = %self.coin,
                amount = %self.amount,
                address = %self.address,
                "Large withdrawal detected - please verify details"
            );
        }

        Ok(())
    }
}

/// Withdraw response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawResponse {
    /// Withdraw ID
    pub id: String,
}

/// Withdraw records response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRecords {
    /// Rows
    pub rows: Vec<WithdrawRecord>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Withdraw record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRecord {
    /// Withdraw ID
    #[serde(default)]
    pub withdraw_id: String,
    /// Tx ID
    #[serde(default)]
    pub tx_i_d: String,
    /// Withdraw type
    #[serde(default)]
    pub withdraw_type: i32,
    /// Coin
    pub coin: String,
    /// Chain
    pub chain: String,
    /// Amount
    pub amount: String,
    /// Withdraw fee
    #[serde(default)]
    pub withdraw_fee: String,
    /// Status
    pub status: String,
    /// To address
    pub to_address: String,
    /// Tag
    #[serde(default)]
    pub tag: String,
    /// Create time
    pub create_time: String,
    /// Update time
    pub update_time: String,
}

/// Withdrawable amount response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmount {
    /// Limit amount info
    #[serde(default)]
    pub limit_amount_usd: String,
    /// Withdrawable amount
    pub withdrawable_amount: WithdrawableAmountDetail,
}

/// Withdrawable amount detail.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmountDetail {
    /// Spot
    #[serde(default)]
    pub s_p_o_t: WithdrawableAmountItem,
    /// Fund
    #[serde(default)]
    pub f_u_n_d: WithdrawableAmountItem,
}

/// Withdrawable amount item.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmountItem {
    /// Coin
    #[serde(default)]
    pub coin: String,
    /// Withdrawable amount
    #[serde(default)]
    pub withdrawable_amount: String,
    /// Available balance
    #[serde(default)]
    pub available_balance: String,
}

/// Cancel withdraw request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelWithdrawParams {
    /// Withdraw ID
    pub id: String,
}
