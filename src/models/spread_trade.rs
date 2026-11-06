use crate::models::{OrderType, Side, TimeInForce};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSpreadOrderParams {
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub qty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendSpreadOrderParams {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelSpreadOrderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllSpreadOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_all: Option<bool>,
}

pub type GetSpreadInstrumentsInfoResponse = SpreadInstrumentsInfoResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadInstrumentsInfoResult {
    #[serde(default)]
    pub list: Option<Vec<SpreadInstrument>>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadInstrument {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub contract_type: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub base_coin: Option<String>,
    #[serde(default)]
    pub quote_coin: Option<String>,
    #[serde(default)]
    pub settle_coin: Option<String>,
    #[serde(default)]
    pub tick_size: Option<String>,
    #[serde(default)]
    pub min_price: Option<String>,
    #[serde(default)]
    pub max_price: Option<String>,
    #[serde(default)]
    pub lot_size: Option<String>,
    #[serde(default)]
    pub min_size: Option<String>,
    #[serde(default)]
    pub max_size: Option<String>,
    #[serde(default)]
    pub launch_time: Option<String>,
    #[serde(default)]
    pub delivery_time: Option<String>,
    #[serde(default)]
    pub legs: Option<Vec<SpreadLeg>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadLeg {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub contract_type: Option<String>,
}

pub type GetSpreadOrderbookResponse = SpreadOrderbookResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadOrderbookResult {
    #[serde(default)]
    pub s: Option<String>,
    #[serde(default)]
    pub b: Option<Vec<GetSpreadOrderbookOrderbookLevel>>,
    #[serde(default)]
    pub a: Option<Vec<GetSpreadOrderbookOrderbookLevel>>,
    #[serde(default)]
    pub ts: Option<i64>,
    #[serde(default)]
    pub u: Option<i64>,
    #[serde(default)]
    pub seq: Option<i64>,
    #[serde(default)]
    pub cts: Option<i64>,
}

pub type GetSpreadOrderbookOrderbookLevel = [String; 2];

pub type GetSpreadRecentTradesResponse = SpreadRecentTradesResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadRecentTradesResult {
    #[serde(default)]
    pub list: Option<Vec<SpreadTradeRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadTradeRecord {
    #[serde(default)]
    pub exec_id: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub side: Option<Side>,
    #[serde(default)]
    pub time: Option<String>,
    #[serde(default)]
    pub seq: Option<String>,
}

pub type GetSpreadTickersResponse = SpreadTickersResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadTickersResult {
    #[serde(default)]
    pub list: Option<Vec<SpreadTicker>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadTicker {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub bid_price: Option<String>,
    #[serde(default)]
    pub bid_size: Option<String>,
    #[serde(default)]
    pub ask_price: Option<String>,
    #[serde(default)]
    pub ask_size: Option<String>,
    #[serde(default)]
    pub last_price: Option<String>,
    #[serde(default)]
    pub high_price24h: Option<String>,
    #[serde(default)]
    pub low_price24h: Option<String>,
    #[serde(default)]
    pub prev_price24h: Option<String>,
    #[serde(default)]
    pub volume24h: Option<String>,
}

pub type GetSpreadMaxQtyResponse = serde_json::Value;
