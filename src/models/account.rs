//! Account models.

use serde::{Deserialize, Serialize};

/// Wallet balance response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    /// List of account balances
    pub list: Vec<AccountBalance>,
}

/// Account balance.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRates {
    /// Category
    pub category: String,
    /// List of fee rates
    pub list: Vec<FeeRate>,
}

/// Fee rate.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogs {
    /// List of transactions
    pub list: Vec<TransactionLog>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Transaction log.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeParams {
    /// Set margin mode
    pub set_margin_mode: String,
}

/// Collateral info response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfo {
    /// List of collateral info
    pub list: Vec<Collateral>,
}

/// Collateral.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistory {
    /// List of borrow records
    pub list: Vec<BorrowRecord>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Borrow record.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub type QueryDcpInfoResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BizDcpInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcp_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<String>,
}

pub type SmpGroupIdQueryByUidResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_fee_rate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupFeeRateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighting_factor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols_numbers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rates: Option<FeeRateDetailMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateDetailMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro: Option<Vec<FeeRateDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_maker: Option<Vec<FeeRateDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_rebate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralRequestItem {
    pub coin: String,
    pub collateral_switch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralParams {
    pub request: Vec<BatchSetCollateralRequestItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralResultItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_switch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<BatchSetCollateralResultItem>>,
}

pub type BatchSetCollateralResponse = BatchSetCollateralResult;

pub type GetAccountInfoResponse = GetAccountInfoResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfoResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_margin_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_master_trader: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_hedging_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcp_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_group: Option<i32>,
}

pub type GetAccountInstrumentsResponse = GetAccountInstrumentsResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInstrumentsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<GetAccountInstrumentsItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInstrumentsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_scale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage_filter: Option<GetAccountInstrumentsLeverageFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_filter: Option<GetAccountInstrumentsPriceFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_size_filter: Option<GetAccountInstrumentsLotSizeFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_margin_trade: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_interval: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_trading: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_funding_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_funding_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_trading: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub st_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_parameters: Option<GetAccountInstrumentsRiskParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub innovation: Option<String>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_listing_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pre_listing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_rpi_permission: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public_rpi: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInstrumentsLeverageFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage_step: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInstrumentsPriceFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInstrumentsLotSizeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_order_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mkt_order_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_order_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty_step: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_notional_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_order_amt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit_order_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_market_order_qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only_max_limit_order_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_order_amt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only_max_order_qty: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInstrumentsRiskParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_limit_ratio_x: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_limit_ratio_y: Option<String>,
}

pub type GetBorrowHistoryResponse = GetBorrowHistoryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBorrowHistoryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<BorrowHistoryItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_borrow_rate: Option<String>,
    #[serde(
        rename = "InterestBearingBorrowSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub interest_bearing_borrow_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_exemption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrealised_loss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_borrowed_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualBorrowParams {
    pub coin: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualRepayParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoConvertRepayParams {
    pub coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneClickRepayParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetMmpParams {
    pub base_coin: String,
}

pub type ResetMmpResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCollateralCoinParams {
    pub coin: String,
    pub collateral_switch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMmpParams {
    pub base_coin: String,
    pub window: String,
    pub frozen_period: String,
    pub qty_limit: String,
    pub delta_limit: String,
}

pub type SetMmpResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPriceLimitParams {
    pub category: String,
    pub modify_enable: bool,
}

pub type SetPriceLimitResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSpotHedgingParams {
    pub set_hedging_mode: String,
}

pub type SetSpotHedgingResponse = serde_json::Value;

pub type UpgradeToUtaProResponse = serde_json::Value;

pub type GetCoinGreeksResponse = GetCoinGreeksResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinGreeksResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<CoinGreeksItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinGreeksItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_delta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_gamma: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_vega: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_theta: Option<String>,
}

pub type GetCollateralInfoResponse = CollateralInfoResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<CollateralInfoItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_borrow_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_borrowing_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_borrowing_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_borrow_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_borrow_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_to_borrow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrowable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_usage_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_collateral: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_switch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_borrowing_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_ratio: Option<String>,
}

pub type GetDcpInfoResponse = GetDcpInfoResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDcpInfoResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcp_infos: Option<Vec<DcpInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DcpInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dcp_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<String>,
}

pub type GetFeeRateResponse = GetFeeRateResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRateResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<GetFeeRateItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRateItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_fee_rate: Option<String>,
}

pub type GetMmpStateResponse = MmpStateResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmpStateResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<MmpStateItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmpStateItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_frozen_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_frozen: Option<bool>,
}

pub type GetSmpGroupResponse = GetSmpGroupResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmpGroupResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_group: Option<i32>,
}

pub type GetTransactionLogResponse = TransactionLogResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TransactionLogEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_time: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_sub_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus_change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fees: Option<String>,
}

pub type GetTransferableAmountResponse = GetTransferableAmountResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferableAmountResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_withdrawal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_withdrawal_map: Option<std::collections::HashMap<String, String>>,
}

pub type GetUserSettingsResponse = GetUserSettingsResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserSettingsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lpa_spot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lpa_perp: Option<bool>,
}

pub type ManualBorrowResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualBorrowResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

pub type ManualRepayResponse = ManualRepayResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualRepayResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_status: Option<String>,
}

pub type NoConvertRepayResponse = NoConvertRepayResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoConvertRepayResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_status: Option<String>,
}

pub type OneClickRepayResponse = OneClickRepayResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneClickRepayResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<OneClickRepayItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OneClickRepayItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repayment_qty: Option<String>,
}

pub type SetMarginModeResponse = SetMarginModeResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<SetMarginModeReason>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeToUtaProResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_update_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_update_msg: Option<UpgradeToUtaProMsg>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeToUtaProMsg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoApplyMoneyParams {
    #[serde(default)]
    pub adjust_type: Option<i64>,
    #[serde(default)]
    pub uta_demo_apply_money: Option<Vec<UtaDemoApplyMoneyConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtaDemoApplyMoneyConfig {
    #[serde(default)]
    pub coin: Option<String>,
    #[serde(default)]
    pub amount_str: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoApplyMoneyResponse {
    #[serde(default)]
    pub result_code: Option<String>,
    #[serde(default)]
    pub uta_demo_apply_money_config: Option<Vec<UtaDemoApplyMoneyConfig>>,
    #[serde(default)]
    pub order_status: Option<String>,
    #[serde(default)]
    pub ret_msg: Option<String>,
}

pub type GetOptionAssetInfoResponse = serde_json::Value;

pub type GetPayInfoResponse = serde_json::Value;

pub type GetTradeInfoForAnalysisResponse = serde_json::Value;
