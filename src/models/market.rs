//! Market data models.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Server time response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    /// Server time in seconds
    pub time_second: String,
    /// Server time in nanoseconds
    pub time_nano: String,
}

/// Instruments info response.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    /// Min price
    pub min_price: Option<String>,
    /// Max price
    pub max_price: Option<String>,
    /// Tick size
    pub tick_size: String,
}

/// Lot size filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tickers {
    /// Category
    pub category: String,
    /// List of tickers
    pub list: Vec<Ticker>,
}

/// Single ticker.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingHistory {
    /// Category
    pub category: String,
    /// List of funding records
    pub list: Vec<FundingRecord>,
}

/// Single funding record.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrades {
    /// Category
    pub category: String,
    /// List of trades
    pub list: Vec<Trade>,
}

/// Single trade.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestRecord {
    /// Open interest
    pub open_interest: String,
    /// Timestamp
    pub timestamp: String,
}

/// Risk limit response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimits {
    /// Category
    pub category: String,
    /// List of risk limits
    pub list: Vec<RiskLimit>,
}

/// Single risk limit.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// ADL alert record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdlAlertRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_pnl_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnl_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adl_trigger_threshold: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adl_stop_ratio: Option<String>,
}

/// ADL alert result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdlAlertResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AdlAlertRecord>>,
}

/// Response payload for the get ADL alert endpoint.
pub type GetAdlAlertResponse = AdlAlertResult;

/// Delivery price record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_time: Option<String>,
}

/// Delivery price result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DeliveryPriceRecord>>,
}

/// Response payload for the get delivery price endpoint.
pub type GetDeliveryPriceResponse = DeliveryPriceResult;

/// Fee rate level record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateLevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_rebate: Option<String>,
}

/// Fee group record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighting_factor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols_numbers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    /// List of taker/maker fee rates by level. Wired to the already-defined
    /// `FeeRateLevel` inner type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rates: Option<Vec<FeeRateLevel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// Fee group info result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeGroupInfoResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<FeeGroup>>,
}

/// Response payload for the get fee group info endpoint.
pub type GetFeeGroupInfoResponse = FeeGroupInfoResult;

/// Historical volatility record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolatilityRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

/// Response payload for the get historical volatility endpoint.
///
/// Bybit V5 returns the inner result as a flat list of volatility records.
/// `client.rs` already unwraps the envelope, so this is just the inner body.
pub type GetHistoricalVolatilityResponse = Vec<VolatilityRecord>;

/// Index component item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexComponentItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

/// Index components result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexComponentsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<IndexComponentItem>>,
}

/// Response payload for the index components endpoint.
pub type IndexComponentsResponse = IndexComponentsResult;

/// Insurance pool record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePoolRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Insurance result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<InsurancePoolRecord>>,
}

/// Response payload for the insurance endpoint.
pub type InsuranceResponse = InsuranceResult;

/// Long/short ratio record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Long/short ratio result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<LongShortRatioRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

/// Response payload for the long/short ratio endpoint.
pub type LongShortRatioResponse = LongShortRatioResult;

/// New delivery price record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDeliveryPriceRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_time: Option<String>,
}

/// New delivery price result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDeliveryPriceResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<NewDeliveryPriceRecord>>,
}

/// Response payload for the new delivery price endpoint.
pub type NewDeliveryPriceResponse = NewDeliveryPriceResult;

/// Order price limit result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPriceLimitResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_lmt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_lmt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
}

pub type OrderPriceLimitResponse = OrderPriceLimitResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpiOrderbookResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<Vec<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<Vec<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cts: Option<i64>,
}

pub type RpiOrderbookResponse = RpiOrderbookResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_pair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equivalent_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

pub type GetIndexPriceComponentsResponse = IndexComponentsResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexKlineEntry {
    pub item: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlineResult {
    pub category: String,
    pub symbol: String,
    pub list: Vec<IndexKlineEntry>,
}

pub type IndexPriceKlineResponse = IndexPriceKlineResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilterLinear {
    pub max_order_qty: String,
    pub min_order_qty: String,
    pub qty_step: String,
    pub post_only_max_order_qty: String,
    pub max_mkt_order_qty: String,
    pub min_notional_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilterSpot {
    pub base_precision: String,
    pub quote_precision: String,
    pub min_order_amt: String,
    pub max_order_amt: String,
    pub max_limit_order_qty: String,
    pub max_market_order_qty: String,
    pub post_only_max_limit_order_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskParameters {
    pub price_limit_ratio_x: String,
    pub price_limit_ratio_y: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentLinearInverse {
    pub symbol: String,
    pub contract_type: String,
    pub status: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub launch_time: String,
    pub delivery_time: String,
    pub delivery_fee_rate: String,
    pub price_scale: String,
    pub leverage_filter: serde_json::Value,
    pub price_filter: serde_json::Value,
    pub lot_size_filter: LotSizeFilterLinear,
    pub unified_margin_trade: bool,
    pub funding_interval: i32,
    pub settle_coin: String,
    pub copy_trading: String,
    pub upper_funding_rate: String,
    pub lower_funding_rate: String,
    pub is_pre_listing: bool,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_listing_info: Option<serde_json::Value>,
    pub risk_parameters: RiskParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentSpot {
    pub symbol: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub status: String,
    pub margin_trading: String,
    pub st_tag: String,
    pub lot_size_filter: LotSizeFilterSpot,
    pub price_filter: serde_json::Value,
    pub risk_parameters: RiskParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentOption {
    pub symbol: String,
    pub options_type: String,
    pub status: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub settle_coin: String,
    pub launch_time: String,
    pub delivery_time: String,
    pub price_filter: serde_json::Value,
    pub lot_size_filter: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfoResult {
    pub category: String,
    pub next_page_cursor: String,
    pub list: Vec<serde_json::Value>,
}

pub type InstrumentsInfoResponse = InstrumentsInfoResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

pub type GetInsurancePoolResponse = InsuranceResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineEntry {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineResult {
    pub category: String,
    pub symbol: String,
    pub list: Vec<KlineEntry>,
}

pub type KlineResponse = KlineResult;

pub type GetLongShortRatioResponse = LongShortRatioResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkKlineEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceKlineResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<MarkKlineEntry>>,
}

pub type MarkPriceKlineResponse = MarkPriceKlineResult;

pub type GetNewDeliveryPriceResponse = NewDeliveryPriceResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<OpenInterestRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

pub type OpenInterestResponse = OpenInterestResult;

pub type GetOrderPriceLimitResponse = OrderPriceLimitResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookLevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<Vec<OrderbookLevel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<Vec<OrderbookLevel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cts: Option<i64>,
}

pub type OrderbookResponse = OrderbookResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexKlineEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexKlineResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<PremiumIndexKlineEntry>>,
}

pub type PremiumIndexKlineResponse = PremiumIndexKlineResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_block_trade: Option<bool>,
    #[serde(rename = "isRPITrade", skip_serializing_if = "Option::is_none")]
    pub is_rpi_trade: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "mP", skip_serializing_if = "Option::is_none")]
    pub m_p: Option<String>,
    #[serde(rename = "iP", skip_serializing_if = "Option::is_none")]
    pub i_p: Option<String>,
    #[serde(rename = "mIv", skip_serializing_if = "Option::is_none")]
    pub m_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TradeRecord>>,
}

pub type RecentTradeResponse = RecentTradeResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimitTier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_limit_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_lowest_risk: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mm_deduction: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimitResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<RiskLimitTier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

pub type RiskLimitResponse = RiskLimitResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpiOrderbookLevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

pub type GetRpiOrderbookResponse = RpiOrderbookResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerLinearInverse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_price_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_24h_pcnt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_price_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_price_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_price_1h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_interest_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turnover_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_funding_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid1_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid1_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask1_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask1_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_interval_hour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_cap: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerSpot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid1_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid1_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask1_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask1_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_price_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_24h_pcnt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_price_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_price_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turnover_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd_index_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid1_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid1_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid1_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask1_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask1_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask1_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_price_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_price_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turnover_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_24h: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vega: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickersResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<serde_json::Value>>,
}

pub type TickersResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResult {
    pub time_second: String,
    pub time_nano: String,
}

pub type ServerTimeResponse = ServerTimeResult;
