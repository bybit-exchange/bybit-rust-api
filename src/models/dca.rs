use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseDcabotParams {
    pub bot_id: i64,
    pub close_mode: DCACloseMode,
}

pub type DCACloseMode = crate::models::OpenApiScalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseDcabotResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub bot_id: Option<i64>,
    #[serde(default)]
    pub close_mode: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDcabotParams {
    pub parameters: BotParameters,
    #[serde(default)]
    pub tools_discovery_parameter: Option<serde_json::Value>,
    #[serde(default)]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotParameters {
    pub frequency_in_second: i64,
    pub quote_coin: String,
    pub pairs: Vec<DCAPair>,
    #[serde(default)]
    pub max_invest_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DCAPair {
    pub base: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDcabotResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub bot_id: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub ban_reason_text: Option<String>,
}
