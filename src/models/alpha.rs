use crate::models::{Interval, OrderType, Side};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLporderListParams {
    #[serde(default)]
    pub order_type: Option<OrderType>,
    #[serde(default)]
    pub token_code: Option<String>,
    #[serde(default)]
    pub order_status: Option<Vec<i64>>,
    #[serde(default)]
    pub days: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub page_index: Option<i64>,
    #[serde(default)]
    pub pool_address: Option<String>,
}

pub type GetLporderListResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppayTokenListParams {
    #[serde(default)]
    pub chain_code: Option<String>,
    #[serde(default)]
    pub token_address: Option<String>,
}

pub type GetLppayTokenListResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppayTokenPriceParams {
    pub token_code: Vec<String>,
    #[serde(default)]
    pub chain_code: Option<String>,
}

pub type GetLppayTokenPriceResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppoolInfoParams {
    pub pool_address: String,
}

pub type GetLppoolInfoResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppoolListParams {
    #[serde(default)]
    pub token_symbol: Option<String>,
}

pub type GetLppoolListResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppositionListParams {}

pub type GetLppositionListResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteLpredeemParams {
    pub position_id: i64,
    pub pool_address: String,
    pub derc_ratio: String,
    #[serde(default)]
    pub receive_token_code: Option<String>,
}

pub type ExecuteLpredeemResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteLpstakeParams {
    pub position_id: i64,
    pub pool_address: String,
    pub pay_token_amount: String,
    pub pay_token_code: String,
    #[serde(default)]
    pub range_upper: Option<String>,
    #[serde(default)]
    pub range_lower: Option<String>,
    #[serde(default)]
    pub price_upper: Option<String>,
    #[serde(default)]
    pub price_lower: Option<String>,
}

pub type ExecuteLpstakeResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutePredictionBuyParams {
    pub token_id: String,
    pub amount: String,
    pub pay_token_code: String,
    pub order_type: OrderType,
    pub slippage: String,
    pub event_id: String,
}

pub type PredictionOrderType = crate::models::OpenApiScalar;

pub type ExecutePredictionBuyResponse = serde_json::Value;

pub type GetPredictionEngineStatusResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionEventDetailParams {
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub has_more_markets: Option<bool>,
}

pub type GetPredictionEventDetailResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionOrderBookParams {
    pub token_ids: Vec<String>,
}

pub type GetPredictionOrderBookResponse = Vec<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionOrderEstimateParams {
    pub token_id: String,
    pub side: Side,
    pub event_id: String,
    pub amount: String,
    pub order_type: OrderType,
    #[serde(default)]
    pub pay_token_code: Option<String>,
}

pub type PredictionSide = crate::models::OpenApiScalar;

pub type GetPredictionOrderEstimateResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionOrderListParams {
    #[serde(default)]
    pub status: Option<PredictionOrderStatus>,
    #[serde(default)]
    pub token_id: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub page_index: Option<i64>,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub side: Option<Side>,
    #[serde(default)]
    pub days: Option<i64>,
}

pub type PredictionOrderStatus = crate::models::OpenApiScalar;

pub type GetPredictionOrderListResponse = serde_json::Value;

pub type GetPredictionPayTokenListResponse = Vec<PredictionPayTokenDto>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictionPayTokenDto {
    #[serde(default)]
    pub token_code: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub token_decimals: Option<i64>,
    #[serde(default)]
    pub token_icon_url_day: Option<String>,
    #[serde(default)]
    pub token_icon_url_night: Option<String>,
    #[serde(default)]
    pub support_chains: Option<Vec<String>>,
}

pub type GetPredictionPortfolioSummaryResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionPositionHistoryParams {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub page_index: Option<i64>,
    #[serde(default)]
    pub direction: Option<String>,
}

pub type GetPredictionPositionHistoryResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionPositionListParams {
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub page_index: Option<i64>,
    #[serde(default)]
    pub direction: Option<String>,
}

pub type GetPredictionPositionListResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionPriceHistoryParams {
    #[serde(default)]
    pub token_ids: Option<Vec<String>>,
    #[serde(default)]
    pub event_id: Option<String>,
    pub interval: Interval,
    #[serde(default)]
    pub fidelity: Option<i64>,
}

pub type PredictionPriceInterval = crate::models::OpenApiScalar;

pub type GetPredictionPriceHistoryResponse = Vec<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutePredictionSellParams {
    pub token_id: String,
    pub size: String,
    pub order_type: OrderType,
    pub slippage: String,
    pub event_id: String,
    #[serde(default)]
    pub to_token_code: Option<String>,
}

pub type ExecutePredictionSellResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionSideMarketListParams {
    pub event_type: PredictionEventType,
}

pub type PredictionEventType = crate::models::OpenApiScalar;

pub type GetPredictionSideMarketListResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionGroupStageDetailParams {
    pub event_type: PredictionEventType,
    pub stage_code: PredictionStageCode,
}

pub type PredictionStageCode = crate::models::OpenApiScalar;

pub type GetPredictionGroupStageDetailResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionMatchListParams {
    pub event_type: PredictionEventType,
}

pub type GetPredictionMatchListResponse = serde_json::Value;

pub type GetPredictionTimelineStagesResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionTokenPriceParams {
    pub token_ids: Vec<String>,
}

pub type GetPredictionTokenPriceResponse = Vec<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetDetailParams {
    pub chain_code: String,
    pub token_address: String,
}

pub type GetAssetDetailResponse = Option<serde_json::Value>;

pub type GetAssetListResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBizTokenDetailsParams {
    pub chain_code: String,
    pub token_address: String,
}

pub type GetBizTokenDetailsResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBizTokenListParams {
    #[serde(default)]
    pub token_tag: Option<i64>,
}

pub type GetBizTokenListResponse = Option<Vec<TradeBizTokenDto>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeBizTokenDto {
    #[serde(default)]
    pub token_code: Option<String>,
    #[serde(default)]
    pub chain_code: Option<String>,
    #[serde(default)]
    pub chain_icon_url: Option<String>,
    #[serde(default)]
    pub token_address: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub token_decimals: Option<i64>,
    #[serde(default)]
    pub token_icon_url_day: Option<String>,
    #[serde(default)]
    pub token_icon_url_night: Option<String>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub create_time_onchain: Option<i64>,
    #[serde(default)]
    pub risk_flag: Option<i64>,
    #[serde(default)]
    pub min_order_quantity: Option<String>,
    #[serde(default)]
    pub max_order_quantity: Option<String>,
    #[serde(default)]
    pub token_tags: Option<Vec<i64>>,
    #[serde(default)]
    pub pay_token_codes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBizTokenPriceListParams {
    pub token_address_info: Vec<TokenAddressInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAddressInfo {
    pub chain_code: String,
    pub token_address: String,
}

pub type GetBizTokenPriceListResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderListParams {
    #[serde(default)]
    pub trade_type: Option<i64>,
    #[serde(default)]
    pub token_code: Option<String>,
    #[serde(default)]
    pub order_status: Option<Vec<i64>>,
    #[serde(default)]
    pub days: Option<i64>,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub page_index: Option<i64>,
    #[serde(default)]
    pub direction: Option<String>,
}

pub type GetOrderListResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPayTokenListParams {
    pub chain_code: String,
    pub token_address: String,
}

pub type GetPayTokenListResponse = Option<Vec<TradePayTokenDto>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradePayTokenDto {
    #[serde(default)]
    pub token_code: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub token_decimals: Option<i64>,
    #[serde(default)]
    pub token_icon_url_day: Option<String>,
    #[serde(default)]
    pub token_icon_url_night: Option<String>,
    #[serde(default)]
    pub limit: Option<String>,
    #[serde(default)]
    pub support_chains: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutePurchaseParams {
    pub from_token_code: String,
    pub from_token_amount: String,
    pub to_token_code: String,
    pub slippage: String,
    pub quote_data: String,
    pub gas: String,
    pub quote_mode: i64,
    pub correcting_code: String,
    #[serde(default)]
    pub tenant: Option<String>,
}

pub type ExecutePurchaseResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeQuoteParams {
    pub trade_type: i64,
    pub from_token_code: String,
    pub from_token_amount: String,
    pub to_token_code: String,
    #[serde(default)]
    pub quote_mode: Option<i64>,
}

pub type GetTradeQuoteResponse = Option<serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRedeemParams {
    pub from_token_code: String,
    pub from_token_amount: String,
    pub to_token_code: String,
    pub slippage: String,
    pub quote_data: String,
    pub gas: String,
    pub quote_mode: i64,
    pub correcting_code: String,
    #[serde(default)]
    pub tenant: Option<String>,
}

pub type ExecuteRedeemResponse = Option<serde_json::Value>;
