use crate::models::Side;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseComboBotParams {
    pub bot_id: i64,
    #[serde(default)]
    pub stop_type: Option<BotStopType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotStopType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseComboBotResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateComboBotParams {
    pub leverage: String,
    pub init_margin: String,
    pub adjust_position_mode: AdjustPositionMode,
    #[serde(default)]
    pub adjust_position_percent: Option<String>,
    #[serde(default)]
    pub adjust_position_time_interval: Option<i64>,
    pub symbol_settings: Vec<SymbolSetting>,
    #[serde(default)]
    pub sl_percent: Option<String>,
    #[serde(default)]
    pub tp_percent: Option<String>,
    #[serde(default)]
    pub source: Option<ComboBotBotSource>,
    #[serde(default)]
    pub block_source: Option<ComboBotBlockSource>,
    #[serde(default)]
    pub create_type: Option<ComboBotCreateType>,
    #[serde(default)]
    pub followed_bot_id: Option<i64>,
    #[serde(default)]
    pub init_bonus: Option<String>,
    #[serde(default)]
    pub trailing_stop_percent: Option<String>,
    #[serde(default)]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustPositionMode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolSetting {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub base_token: Option<String>,
    #[serde(default)]
    pub quote_token: Option<String>,
    #[serde(default)]
    pub target_position_percent: Option<String>,
    #[serde(default)]
    pub side: Option<Side>,
    #[serde(default)]
    pub symbol_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboBotBotSource {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboBotBlockSource {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboBotCreateType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateComboBotResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub bot_id: Option<i64>,
    #[serde(default)]
    pub ban_reason_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetComboDetailParams {
    pub bot_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetComboDetailResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub detail: Option<BotDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotDetail {
    #[serde(default)]
    pub bot_id: Option<i64>,
    #[serde(default)]
    pub bot_mode: Option<BotMode>,
    #[serde(default)]
    pub bot_display_status: Option<BotDisplayStatus>,
    #[serde(default)]
    pub leverage: Option<String>,
    #[serde(default)]
    pub total_margin: Option<String>,
    #[serde(default)]
    pub total_pnl: Option<String>,
    #[serde(default)]
    pub total_pnl_per: Option<String>,
    #[serde(default)]
    pub equity: Option<String>,
    #[serde(default)]
    pub roi: Option<String>,
    #[serde(default)]
    pub total_apr: Option<String>,
    #[serde(default)]
    pub adjusted_position_num: Option<i64>,
    #[serde(default)]
    pub last_adjust_position_time: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub close_code: Option<ComboBotBotCloseCode>,
    #[serde(default)]
    pub ws_token: Option<String>,
    #[serde(default)]
    pub symbol_settings: Option<Vec<SymbolSettingComboBotGetDetail>>,
    #[serde(default)]
    pub sl_percent: Option<String>,
    #[serde(default)]
    pub tp_percent: Option<String>,
    #[serde(default)]
    pub total_position_value: Option<String>,
    #[serde(default)]
    pub margin_balance: Option<String>,
    #[serde(default)]
    pub available_balance: Option<String>,
    #[serde(default)]
    pub imr: Option<String>,
    #[serde(default)]
    pub mmr: Option<String>,
    #[serde(default)]
    pub funding_fee: Option<String>,
    #[serde(default)]
    pub adjust_position_mode: Option<AdjustPositionModeComboBotGetDetail>,
    #[serde(default)]
    pub adjust_position_percent: Option<String>,
    #[serde(default)]
    pub adjust_position_time_interval: Option<i64>,
    #[serde(default)]
    pub bot_name: Option<String>,
    #[serde(default)]
    pub run_time_duration: Option<i64>,
    #[serde(default)]
    pub settlement_assets: Option<String>,
    #[serde(default)]
    pub stop_type: Option<BotStopTypeComboBotGetDetail>,
    #[serde(default)]
    pub follow_num: Option<i64>,
    #[serde(default)]
    pub used_reward_amount: Option<String>,
    #[serde(default)]
    pub used_reward_id: Option<String>,
    #[serde(default)]
    pub total_bonus: Option<String>,
    #[serde(default)]
    pub realized_pnl: Option<String>,
    #[serde(default)]
    pub unrealized_pnl: Option<String>,
    #[serde(default)]
    pub total_exec_fee: Option<String>,
    #[serde(default)]
    pub last_modify_setting_time: Option<i64>,
    #[serde(default)]
    pub midway_transfer: Option<String>,
    #[serde(default)]
    pub exit_equity: Option<String>,
    #[serde(default)]
    pub trailing_stop_percent: Option<String>,
    #[serde(default)]
    pub sl_exit_equity: Option<String>,
    #[serde(default)]
    pub tp_exit_equity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotMode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotDisplayStatus {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboBotBotCloseCode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolSettingComboBotGetDetail {
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub base_token: Option<String>,
    #[serde(default)]
    pub quote_token: Option<String>,
    #[serde(default)]
    pub target_position_percent: Option<String>,
    #[serde(default)]
    pub side: Option<Side>,
    #[serde(default)]
    pub symbol_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SideComboBotGetDetail {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustPositionModeComboBotGetDetail {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotStopTypeComboBotGetDetail {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetComboLimitParams {
    pub leverage: String,
    pub init_margin: String,
    pub adjust_position_mode: AdjustPositionMode,
    #[serde(default)]
    pub adjust_position_percent: Option<String>,
    #[serde(default)]
    pub adjust_position_time_interval: Option<i64>,
    pub symbol_settings: Vec<SymbolSetting>,
    #[serde(default)]
    pub sl_percent: Option<String>,
    #[serde(default)]
    pub tp_percent: Option<String>,
    #[serde(default)]
    pub need_to_slippage: Option<bool>,
    #[serde(default)]
    pub app_name: Option<String>,
    #[serde(default)]
    pub trailing_stop_percent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetComboLimitResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub check_code: Option<LimitCheckCode>,
    #[serde(default)]
    pub adjust_position_percent: Option<ComboBotRange>,
    #[serde(default)]
    pub adjust_position_time_interval: Option<ComboBotRange>,
    #[serde(default)]
    pub init_margin: Option<ComboBotRange>,
    #[serde(default)]
    pub sl_percent: Option<ComboBotRange>,
    #[serde(default)]
    pub tp_percent: Option<ComboBotRange>,
    #[serde(default)]
    pub leverage: Option<ComboBotRange>,
    #[serde(default)]
    pub trailing_stop_percent: Option<ComboBotRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitCheckCode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboBotRange {
    #[serde(default)]
    pub min: Option<String>,
    #[serde(default)]
    pub max: Option<String>,
}
