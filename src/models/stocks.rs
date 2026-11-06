use crate::models::{AccountType, Side, TimeInForce};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceStocksOrderParams {
    pub symbol: String,
    pub quote_token: String,
    pub side: Side,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub qty: Option<String>,
    #[serde(default)]
    pub notional: Option<String>,
    #[serde(default)]
    pub limit_price: Option<String>,
    #[serde(default)]
    pub stop_price: Option<String>,
    pub time_in_force: TimeInForce,
    #[serde(default)]
    pub trading_session: Option<String>,
    #[serde(default)]
    pub tokenize: Option<bool>,
    pub order_time: i64,
    pub request_id: String,
}

pub type PlaceStocksOrderResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelStocksOrderParams {
    pub order_no: String,
}

pub type CancelStocksOrderResponse = Option<serde_json::Value>;

pub type GetStocksOrderDetailResponse = Option<OrderDetailDto>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetailDto {
    #[serde(default)]
    pub order_no: Option<String>,
    #[serde(default)]
    pub request_id: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub quote_token: Option<String>,
    #[serde(default)]
    pub side: Option<Side>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_: Option<String>,
    #[serde(default)]
    pub qty: Option<String>,
    #[serde(default)]
    pub notional: Option<String>,
    #[serde(default)]
    pub limit_price: Option<String>,
    #[serde(default)]
    pub stop_price: Option<String>,
    #[serde(default)]
    pub time_in_force: Option<TimeInForce>,
    #[serde(default)]
    pub trading_session: Option<String>,
    #[serde(default)]
    pub tokenize: Option<bool>,
    #[serde(default)]
    pub order_status: Option<OrderDetailStatus>,
    #[serde(default)]
    pub filled_qty: Option<String>,
    #[serde(default)]
    pub filled_notional: Option<String>,
    #[serde(default)]
    pub avg_price: Option<String>,
    #[serde(default)]
    pub fail_reason: Option<String>,
    #[serde(default)]
    pub order_time: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

pub type OrderDetailStatus = crate::models::OpenApiScalar;

pub type GetConvertListResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitConvertParams {
    pub convert_type: String,
    pub symbol: String,
    pub input_amount: String,
    pub front_multiplier: String,
    #[serde(default)]
    pub account_type: Option<AccountType>,
    pub request_id: String,
    #[serde(default)]
    pub contract_addr: Option<String>,
    pub flow: String,
    #[serde(default)]
    pub burn_scene: Option<String>,
}

pub type SubmitConvertResponse = Option<serde_json::Value>;

pub type GetConvertDetailResponse = Option<ConvertDetailDto>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertDetailDto {
    #[serde(default)]
    pub order_no: Option<String>,
    #[serde(default)]
    pub request_id: Option<String>,
    #[serde(default)]
    pub status: Option<ConvertStatus>,
    #[serde(default)]
    pub convert_type: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub input_amount: Option<String>,
    #[serde(default)]
    pub output_amount: Option<String>,
    #[serde(default)]
    pub applied_multiplier: Option<String>,
    #[serde(default)]
    pub account_type: Option<AccountType>,
    #[serde(default)]
    pub fail_reason: Option<String>,
    #[serde(default)]
    pub submitted_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,
}

pub type ConvertStatus = crate::models::OpenApiScalar;

pub type GetStocksPositionsResponse = Option<serde_json::Value>;

pub type GetMarketSessionResponse = Option<MarketSessionDto>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketSessionDto {
    #[serde(default)]
    pub symbol_type: Option<String>,
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub server_time: Option<i64>,
    #[serde(default)]
    pub current_phase_end_time: Option<i64>,
    #[serde(default)]
    pub next_market_open_time: Option<i64>,
    #[serde(default)]
    pub stocks_trading_holiday: Option<Vec<TradingHolidayDto>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingHolidayDto {
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_: Option<String>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
}

pub type GetMultiplierListResponse = Option<MultiplierListResult>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiplierListResult {
    #[serde(default)]
    pub current: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub list: Option<Vec<MultiplierItemDto>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiplierItemDto {
    #[serde(default)]
    pub mstock_symbol: Option<String>,
    #[serde(default)]
    pub multiplier: Option<String>,
    #[serde(default)]
    pub effective_time: Option<i64>,
}
