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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLporderListResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppayTokenListParams {
    #[serde(default)]
    pub chain_code: Option<String>,
    #[serde(default)]
    pub token_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppayTokenListResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppayTokenPriceParams {
    pub token_code: Vec<String>,
    #[serde(default)]
    pub chain_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppayTokenPriceResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppoolInfoParams {
    pub pool_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppoolInfoResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppoolListParams {
    #[serde(default)]
    pub token_symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppoolListResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppositionListParams {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLppositionListResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteLpredeemParams {
    pub position_id: i64,
    pub pool_address: String,
    pub derc_ratio: String,
    #[serde(default)]
    pub receive_token_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteLpredeemResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteLpstakeResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictionOrderType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutePredictionBuyResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionEngineStatusResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionEventDetailResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionOrderBookParams {
    pub token_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionOrderBookResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: Vec<serde_json::Value>,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictionSide {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionOrderEstimateResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictionOrderStatus {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionOrderListResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionPayTokenListResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: Vec<PredictionPayTokenDto>,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionPortfolioSummaryResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionPositionHistoryResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionPositionListResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictionPriceInterval {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionPriceHistoryResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: Vec<serde_json::Value>,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutePredictionSellResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionSideMarketListParams {
    pub event_type: PredictionEventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictionEventType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionSideMarketListResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionGroupStageDetailParams {
    pub event_type: PredictionEventType,
    pub stage_code: PredictionStageCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictionStageCode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionGroupStageDetailResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionMatchListParams {
    pub event_type: PredictionEventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionMatchListResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionTimelineStagesResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionTokenPriceParams {
    pub token_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPredictionTokenPriceResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: Vec<serde_json::Value>,
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetDetailParams {
    pub chain_code: String,
    pub token_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetDetailResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetListResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBizTokenDetailsParams {
    pub chain_code: String,
    pub token_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBizTokenDetailsResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBizTokenListParams {
    #[serde(default)]
    pub token_tag: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBizTokenListResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<Vec<TradeBizTokenDto>>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBizTokenPriceListResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderListResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPayTokenListParams {
    pub chain_code: String,
    pub token_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPayTokenListResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<Vec<TradePayTokenDto>>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutePurchaseResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradeQuoteResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRedeemResponse {
    #[serde(default)]
    pub ret_code: Option<i64>,
    #[serde(default)]
    pub ret_msg: Option<String>,
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub ret_ext_info: Option<serde_json::Value>,
    #[serde(default)]
    pub time: Option<i64>,
}
