//! Asset models.

use crate::error::{BybitError, Result};
use crate::models::AccountType;
use serde::{Deserialize, Serialize};
use tracing::warn;

/// Coin info response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfoResponse {
    /// List of coins
    pub rows: Vec<CoinInfo>,
}

/// Coin info.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResponse {
    /// Transfer ID
    pub transfer_id: String,
}

/// Transfer list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferList {
    /// List of transfers
    pub list: Vec<TransferRecord>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Transfer record.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddressResponse {
    /// Coin
    pub coin: String,
    /// Chains
    pub chains: Vec<DepositChainAddress>,
}

/// Deposit chain address.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecords {
    /// Rows
    pub rows: Vec<DepositRecord>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Deposit record.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawResponse {
    /// Withdraw ID
    pub id: String,
}

/// Withdraw records response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRecords {
    /// Rows
    pub rows: Vec<WithdrawRecord>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Withdraw record.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmount {
    /// Limit amount info
    #[serde(default)]
    pub limit_amount_usd: String,
    /// Withdrawable amount
    pub withdrawable_amount: WithdrawableAmountDetail,
}

/// Withdrawable amount detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelWithdrawParams {
    /// Withdraw ID
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultDepositToAccountParams {
    pub account_type: AccountType,
}

pub type SetDefaultDepositToAccountResponse = serde_json::Value;
pub type QueryInternalDepositRecordsResponse = serde_json::Value;
pub type QuerySubMemberDepositAddressResponse = serde_json::Value;
pub type QuerySubMemberDepositRecordsResponse = serde_json::Value;
pub type AccountCoinBalanceQueryResponse = serde_json::Value;
pub type AssetInfoQueryResponse = serde_json::Value;
pub type SubMemberListQueryResponse = serde_json::Value;
pub type TransferCoinListQueryResponse = serde_json::Value;
pub type UniversalTransferListQueryResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferSubMemberSaveParams {
    pub sub_member_ids: String,
}

pub type TransferSubMemberSaveResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferParams {
    pub transfer_id: String,
    pub coin: String,
    pub amount: String,
    pub from_member_id: i64,
    pub to_member_id: i64,
    pub from_account_type: AccountType,
    pub to_account_type: AccountType,
}

pub type UniversalTransferResponse = serde_json::Value;
pub type QueryWithdrawAddressesResponse = serde_json::Value;
pub type GetVasplistResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetOverviewResponse {
    #[serde(default)]
    pub total_equity: Option<String>,
    #[serde(default)]
    pub list: Option<Vec<AssetOverviewAccountItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetOverviewAccountItem {
    #[serde(default)]
    pub account_type: Option<String>,
    #[serde(default)]
    pub snapshot_time: Option<String>,
    #[serde(default)]
    pub valuation_currency: Option<String>,
    #[serde(default)]
    pub total_equity: Option<String>,
    #[serde(default)]
    pub categories: Option<Vec<AssetOverviewCategory>>,
    #[serde(default)]
    pub coin_detail: Option<Vec<AssetOverviewCoinDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetOverviewCategory {
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub equity: Option<String>,
    #[serde(default)]
    pub coin_detail: Option<Vec<AssetOverviewCoinDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetOverviewCoinDetail {
    #[serde(default)]
    pub coin: Option<String>,
    #[serde(default)]
    pub equity: Option<String>,
    #[serde(default)]
    pub ext_map: Option<serde_json::Value>,
}

pub type GetDeliveryRecordResponse = serde_json::Value;
pub type GetPortfolioMarginResponse = serde_json::Value;
pub type GetSettlementRecordResponse = serde_json::Value;
pub type GetTotalMembersAssetsResponse = serde_json::Value;

/// Wallet type used by the Convert APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConvertAccountType {
    #[serde(rename = "eb_convert_funding")]
    Funding,
    #[serde(rename = "eb_convert_uta")]
    Uta,
    #[serde(rename = "eb_convert_spot")]
    Spot,
    #[serde(rename = "eb_convert_contract")]
    Contract,
    #[serde(rename = "eb_convert_inverse")]
    Inverse,
}

impl std::fmt::Display for ConvertAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Funding => "eb_convert_funding",
            Self::Uta => "eb_convert_uta",
            Self::Spot => "eb_convert_spot",
            Self::Contract => "eb_convert_contract",
            Self::Inverse => "eb_convert_inverse",
        })
    }
}

/// Which side of the Convert coin list to return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertCoinSide {
    /// Coins available to sell (`side=0`).
    FromCoin,
    /// Coins available to buy (`side=1`).
    ToCoin,
}

impl std::fmt::Display for ConvertCoinSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::FromCoin => "0",
            Self::ToCoin => "1",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinListQueryResponse {
    #[serde(default)]
    pub coins: Option<Vec<CoinListQueryCoinInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinListQueryCoinInfo {
    #[serde(default)]
    pub coin: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub icon_night: Option<String>,
    #[serde(default)]
    pub accuracy_length: Option<i64>,
    #[serde(default)]
    pub coin_type: Option<String>,
    #[serde(default)]
    pub balance: Option<String>,
    #[serde(default)]
    pub u_balance: Option<String>,
    #[serde(default)]
    pub single_from_min_limit: Option<String>,
    #[serde(default)]
    pub single_from_max_limit: Option<String>,
    #[serde(default)]
    pub disable_from: Option<bool>,
    #[serde(default)]
    pub disable_to: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertExecuteParams {
    pub quote_tx_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertExecuteResponse {
    #[serde(default)]
    pub exchange_status: Option<String>,
    #[serde(default)]
    pub quote_tx_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertHistoryQueryResponse {
    #[serde(default)]
    pub list: Option<Vec<ConvertResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertResult {
    #[serde(default)]
    pub account_type: Option<String>,
    #[serde(default)]
    pub exchange_tx_id: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub from_coin: Option<String>,
    #[serde(default)]
    pub from_coin_type: Option<String>,
    #[serde(default)]
    pub from_amount: Option<String>,
    #[serde(default)]
    pub to_coin: Option<String>,
    #[serde(default)]
    pub to_coin_type: Option<String>,
    #[serde(default)]
    pub to_amount: Option<String>,
    #[serde(default)]
    pub exchange_status: Option<String>,
    #[serde(default)]
    pub ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub convert_rate: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResultResponse {
    #[serde(default)]
    pub result: Option<ConvertResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderByPageResponse {
    #[serde(default)]
    pub order_body: Option<Vec<ExchangeBody>>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeBody {
    #[serde(default)]
    pub from_coin: Option<String>,
    #[serde(default)]
    pub from_amount: Option<String>,
    #[serde(default)]
    pub to_coin: Option<String>,
    #[serde(default)]
    pub to_amount: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub exchange_tx_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteApplyParams {
    #[serde(default)]
    pub request_id: Option<String>,
    pub account_type: ConvertAccountType,
    pub from_coin: String,
    #[serde(default)]
    pub from_coin_type: Option<String>,
    pub to_coin: String,
    #[serde(default)]
    pub to_coin_type: Option<String>,
    pub request_amount: String,
    pub request_coin: String,
    #[serde(default)]
    pub param_type: Option<String>,
    #[serde(default)]
    pub param_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteApplyResponse {
    #[serde(default)]
    pub quote_tx_id: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<String>,
    #[serde(default)]
    pub from_coin: Option<String>,
    #[serde(default)]
    pub from_coin_type: Option<String>,
    #[serde(default)]
    pub to_coin: Option<String>,
    #[serde(default)]
    pub to_coin_type: Option<String>,
    #[serde(default)]
    pub from_amount: Option<String>,
    #[serde(default)]
    pub to_amount: Option<String>,
    #[serde(default)]
    pub expired_time: Option<String>,
    #[serde(default)]
    pub request_id: Option<String>,
    #[serde(default)]
    pub ext_tax_and_fee: Option<Vec<TaxAndFee>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxAndFee {
    #[serde(default)]
    pub tax_fee_name: Option<String>,
    #[serde(default)]
    pub tax_fee_rate: Option<String>,
    #[serde(default)]
    pub tax_fee_amount: Option<String>,
    #[serde(default)]
    pub tax_fee_coin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallAssetConvertParams {
    pub quote_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallAssetConvertResponse {
    #[serde(default)]
    pub quote_id: Option<String>,
    #[serde(default)]
    pub exchange_tx_id: Option<String>,
    #[serde(default)]
    pub submit_time: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySmallAssetConvertOrderResponse {
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub last_page: Option<String>,
    #[serde(default)]
    pub total_count: Option<String>,
    #[serde(default)]
    pub records: Option<Vec<MainConvertRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainConvertRecord {
    #[serde(default)]
    pub account_type: Option<String>,
    #[serde(default)]
    pub exchange_tx_id: Option<String>,
    #[serde(default)]
    pub to_coin: Option<String>,
    #[serde(default)]
    pub to_amount: Option<String>,
    #[serde(default)]
    pub sub_records: Option<Vec<SubConvertRecord>>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub exchange_source: Option<String>,
    #[serde(default)]
    pub fee_coin: Option<String>,
    #[serde(default)]
    pub total_fee_amount: Option<String>,
    #[serde(default)]
    pub total_tax_fee_info: Option<TaxFeeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubConvertRecord {
    #[serde(default)]
    pub from_coin: Option<String>,
    #[serde(default)]
    pub from_amount: Option<String>,
    #[serde(default)]
    pub to_coin: Option<String>,
    #[serde(default)]
    pub to_amount: Option<String>,
    #[serde(default)]
    pub fee_coin: Option<String>,
    #[serde(default)]
    pub fee_amount: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub tax_fee_info: Option<TaxFeeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxFeeInfo {
    #[serde(default)]
    pub total_amount: Option<String>,
    #[serde(default)]
    pub fee_coin: Option<String>,
    #[serde(default)]
    pub tax_fee_items: Option<Vec<TaxFeeItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxFeeItem {
    #[serde(default)]
    pub fee_type: Option<String>,
    #[serde(default)]
    pub fee_rate: Option<String>,
    #[serde(default)]
    pub fee_amount: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub fee_coin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySmallAssetListResponse {
    #[serde(default)]
    pub small_asset_coins: Option<Vec<SmallAssetCoin>>,
    #[serde(default)]
    pub support_to_coins: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallAssetCoin {
    #[serde(default)]
    pub from_coin: Option<String>,
    #[serde(default)]
    pub support_convert: Option<i64>,
    #[serde(default)]
    pub available_balance: Option<String>,
    #[serde(default)]
    pub base_value: Option<String>,
    #[serde(default)]
    pub to_coin: Option<String>,
    #[serde(default)]
    pub to_amount: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<String>,
    #[serde(default)]
    pub fee_info: Option<serde_json::Value>,
    #[serde(default)]
    pub tax_fee_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallAssetQuoteParams {
    pub account_type: ConvertAccountType,
    pub to_coin: String,
    pub from_coin_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallAssetQuoteResponse {
    #[serde(default)]
    pub quote_id: Option<String>,
    #[serde(default)]
    pub result: Option<SmallAssetQuoteResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallAssetQuoteResult {
    #[serde(default)]
    pub quote_create_time: Option<String>,
    #[serde(default)]
    pub quote_expire_time: Option<String>,
    #[serde(default)]
    pub exchange_coins: Option<Vec<SmallAssetCoinSmallBalanceQuote>>,
    #[serde(default)]
    pub total_fee_info: Option<FeeInfo>,
    #[serde(default)]
    pub total_tax_fee_info: Option<TaxFeeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallAssetCoinSmallBalanceQuote {
    #[serde(default)]
    pub from_coin: Option<String>,
    #[serde(default)]
    pub support_convert: Option<i64>,
    #[serde(default)]
    pub available_balance: Option<String>,
    #[serde(default)]
    pub base_value: Option<String>,
    #[serde(default)]
    pub to_coin: Option<String>,
    #[serde(default)]
    pub to_amount: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<String>,
    #[serde(default)]
    pub fee_info: Option<FeeInfo>,
    #[serde(default)]
    pub tax_fee_info: Option<TaxFeeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeInfo {
    #[serde(default)]
    pub fee_coin: Option<String>,
    #[serde(default)]
    pub amount: Option<String>,
    #[serde(default)]
    pub fee_rate: Option<String>,
}
