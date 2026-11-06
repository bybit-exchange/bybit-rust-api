use crate::models::AccountType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseGridBotParams {
    pub grid_id: i64,
    pub close_mode: GridCloseMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridCloseMode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseGridBotResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGridBotParams {
    pub symbol: String,
    pub max_price: String,
    pub min_price: String,
    pub total_investment: String,
    pub cell_number: i64,
    #[serde(default)]
    pub followed_grid_id: Option<i64>,
    #[serde(default)]
    pub source: Option<GridBotSource>,
    #[serde(default)]
    pub entry_price: Option<String>,
    #[serde(default)]
    pub stop_loss_price: Option<String>,
    #[serde(default)]
    pub take_profit_price: Option<String>,
    #[serde(default)]
    pub tools_discovery_parameter: Option<serde_json::Value>,
    #[serde(default)]
    pub base_investment: Option<String>,
    #[serde(default)]
    pub quote_investment: Option<String>,
    #[serde(default)]
    pub invest_mode: Option<InvestMode>,
    #[serde(default)]
    pub block_source: Option<GridBlockSource>,
    #[serde(default)]
    pub create_type: Option<GridCreateType>,
    #[serde(default)]
    pub ts_percent: Option<String>,
    #[serde(default)]
    pub enable_trailing: Option<bool>,
    #[serde(default)]
    pub limit_up_price: Option<String>,
    #[serde(default)]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridBotSource {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvestMode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridBlockSource {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridCreateType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGridBotResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub grid_id: Option<i64>,
    #[serde(default)]
    pub ban_reason_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryGridDetailParams {
    pub grid_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryGridDetailResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub detail: Option<GridDetail>,
    #[serde(default)]
    pub debug_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridDetail {
    #[serde(default)]
    pub grid_id: Option<i64>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub base_token: Option<String>,
    #[serde(default)]
    pub quote_token: Option<String>,
    #[serde(default)]
    pub total_investment: Option<String>,
    #[serde(default)]
    pub total_profit: Option<String>,
    #[serde(default)]
    pub max_price: Option<String>,
    #[serde(default)]
    pub min_price: Option<String>,
    #[serde(default)]
    pub cell_number: Option<i64>,
    #[serde(default)]
    pub grid_profit: Option<String>,
    #[serde(default)]
    pub grid_apr: Option<String>,
    #[serde(default)]
    pub total_apr: Option<String>,
    #[serde(default)]
    pub arbitrage_num: Option<i64>,
    #[serde(default)]
    pub arbitrage_num_24: Option<i64>,
    #[serde(default)]
    pub allow_follow: Option<i64>,
    #[serde(default)]
    pub operation_time: Option<i64>,
    #[serde(default)]
    pub status: Option<GridStatus>,
    #[serde(default)]
    pub entry_price: Option<String>,
    #[serde(default)]
    pub stop_loss_price: Option<String>,
    #[serde(default)]
    pub take_profit_price: Option<String>,
    #[serde(default)]
    pub current_price: Option<String>,
    #[serde(default)]
    pub close_reason: Option<GridCloseReason>,
    #[serde(default)]
    pub bot_close_code: Option<GridBotCloseCode>,
    #[serde(default)]
    pub follow_num: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub used_reward_amount: Option<String>,
    #[serde(default)]
    pub settlement_assets: Option<String>,
    #[serde(default)]
    pub account_type: Option<AccountType>,
    #[serde(default)]
    pub cum_withdrew_amount: Option<String>,
    #[serde(default)]
    pub current_profit: Option<String>,
    #[serde(default)]
    pub current_per: Option<String>,
    #[serde(default)]
    pub equity: Option<String>,
    #[serde(default)]
    pub ts_percent: Option<String>,
    #[serde(default)]
    pub is_support_ts: Option<bool>,
    #[serde(default)]
    pub enable_trailing: Option<bool>,
    #[serde(default)]
    pub limit_up_price: Option<String>,
    #[serde(default)]
    pub ori_max_price: Option<String>,
    #[serde(default)]
    pub ori_min_price: Option<String>,
    #[serde(default)]
    pub ori_cell_number: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridStatus {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridCloseReason {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridBotCloseCode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateGridInputParams {
    pub symbol: String,
    pub cell_number: i64,
    pub min_price: String,
    pub max_price: String,
    pub total_investment: String,
    #[serde(default)]
    pub stop_loss: Option<String>,
    #[serde(default)]
    pub take_profit: Option<String>,
    #[serde(default)]
    pub entry_price: Option<String>,
    #[serde(default)]
    pub base_investment: Option<String>,
    #[serde(default)]
    pub quote_investment: Option<String>,
    #[serde(default)]
    pub invest_mode: Option<InvestMode>,
    #[serde(default)]
    pub ts_percent: Option<String>,
    #[serde(default)]
    pub enable_trailing: Option<bool>,
    #[serde(default)]
    pub limit_up_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateGridInputResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub investment: Option<GridRange>,
    #[serde(default)]
    pub profit: Option<GridRange>,
    #[serde(default)]
    pub cell_number: Option<GridRange>,
    #[serde(default)]
    pub min_price: Option<GridRange>,
    #[serde(default)]
    pub max_price: Option<GridRange>,
    #[serde(default)]
    pub stop_loss: Option<GridRange>,
    #[serde(default)]
    pub take_profit: Option<GridRange>,
    #[serde(default)]
    pub entry_price: Option<GridRange>,
    #[serde(default)]
    pub cell_distance: Option<String>,
    #[serde(default)]
    pub check_code: Option<ValidateInputCheckCode>,
    #[serde(default)]
    pub base_investment: Option<GridRange>,
    #[serde(default)]
    pub ts_percent: Option<GridRange>,
    #[serde(default)]
    pub limit_up_price: Option<GridRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridRange {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateInputCheckCode {}
