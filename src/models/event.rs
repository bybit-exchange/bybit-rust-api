use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcOrderHistoryResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcActiveOrdersResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcPositionInfoResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcSettlementsResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEcTradeHistoryResponse {
    pub ret_code: i64,
    pub ret_msg: String,
    pub result: serde_json::Value,
}
