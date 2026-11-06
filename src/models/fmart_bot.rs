use serde::{Deserialize, Serialize};

// NOTE: Bybit V5 endpoints universally use camelCase on the wire (e.g.
// `orderId`, `accountType`). The container-level `rename_all = "camelCase"`
// attribute drives wire serialization here. The previous version of this
// file also had per-field `#[serde(rename = "snake_case")]` attributes
// which silently *overrode* the container, sending snake_case keys —
// inconsistent with the rest of the V5 SDK. They have been removed.

/// Response payload for `POST /v5/fmartingalebot/create`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFMartBotResponse {
    #[serde(default)]
    pub status_code: Option<i32>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub ban_reason_text: Option<String>,
    #[serde(default)]
    pub bot_id: Option<i64>,
}

/// Request body for `POST /v5/fmartingalebot/create`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFMartBotParams {
    pub symbol: String,
    pub martingale_mode: String,
    pub leverage: String,
    pub price_float_percent: String,
    pub add_position_percent: String,
    pub add_position_num: i32,
    pub init_margin: String,
    pub round_tp_percent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_cycle_toggle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_percent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followed_bot_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_bonus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseFmartBotParams {
    pub bot_id: i64,
    #[serde(default)]
    pub stop_type: Option<FMartBotStopType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMartBotStopType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseFmartBotResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFmartDetailParams {
    pub bot_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFmartDetailResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub fmart_detail: Option<FMartBotDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMartBotDetail {
    #[serde(default)]
    pub bot_id: Option<i64>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub bot_display_status: Option<FMartBotDisplayStatus>,
    #[serde(default)]
    pub fmart_mode: Option<FMartMode>,
    #[serde(default)]
    pub base_token: Option<String>,
    #[serde(default)]
    pub quote_token: Option<String>,
    #[serde(default)]
    pub leverage: Option<String>,
    #[serde(default)]
    pub single_round_target_profit_per: Option<String>,
    #[serde(default)]
    pub add_pos_num: Option<String>,
    #[serde(default)]
    pub add_pos_per: Option<String>,
    #[serde(default)]
    pub price_float_per: Option<String>,
    #[serde(default)]
    pub entry_price: Option<String>,
    #[serde(default)]
    pub sl_per: Option<String>,
    #[serde(default)]
    pub total_margin: Option<String>,
    #[serde(default)]
    pub pnl: Option<String>,
    #[serde(default)]
    pub pnl_per: Option<String>,
    #[serde(default)]
    pub realized_pnl: Option<String>,
    #[serde(default)]
    pub unrealized_pnl: Option<String>,
    #[serde(default)]
    pub completed_rounds: Option<i64>,
    #[serde(default)]
    pub current_round: Option<i64>,
    #[serde(default)]
    pub current_added_pos_num: Option<i64>,
    #[serde(default)]
    pub pos_size: Option<String>,
    #[serde(default)]
    pub avg_pos_price: Option<String>,
    #[serde(default)]
    pub pos_balance: Option<String>,
    #[serde(default)]
    pub order_balance: Option<String>,
    #[serde(default)]
    pub available_balance: Option<String>,
    #[serde(default)]
    pub mark_price: Option<String>,
    #[serde(default)]
    pub estimated_liq_price: Option<String>,
    #[serde(default)]
    pub actual_leverage: Option<String>,
    #[serde(default)]
    pub auto_cycle: Option<AutoCycleToggle>,
    #[serde(default)]
    pub operation_time: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub modify_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub stop_type: Option<FMartBotStopType>,
    #[serde(default)]
    pub settlement_assets: Option<String>,
    #[serde(default)]
    pub tick_size: Option<String>,
    #[serde(default)]
    pub close_price: Option<String>,
    #[serde(default)]
    pub current_price: Option<String>,
    #[serde(default)]
    pub running_duration: Option<i64>,
    #[serde(default)]
    pub close_code: Option<FmartBotBotCloseCode>,
    #[serde(default)]
    pub net_funding_fee: Option<String>,
    #[serde(default)]
    pub total_apr: Option<String>,
    #[serde(default)]
    pub follow_num: Option<i64>,
    #[serde(default)]
    pub used_reward_amount: Option<String>,
    #[serde(default)]
    pub used_reward_id: Option<String>,
    #[serde(default)]
    pub total_bonus: Option<String>,
    #[serde(default)]
    pub adl_rank_indicator: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMartBotDisplayStatus {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMartMode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCycleToggle {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FmartBotBotCloseCode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFmartLimitParams {
    pub symbol: String,
    pub martingale_mode: FMartMode,
    pub leverage: String,
    #[serde(default)]
    pub price_float_percent: Option<String>,
    #[serde(default)]
    pub add_position_percent: Option<String>,
    #[serde(default)]
    pub add_position_num: Option<i64>,
    #[serde(default)]
    pub init_margin: Option<String>,
    #[serde(default)]
    pub round_tp_percent: Option<String>,
    #[serde(default)]
    pub sl_percent: Option<String>,
    #[serde(default)]
    pub entry_price: Option<String>,
    #[serde(default)]
    pub need_to_slippage: Option<bool>,
    #[serde(default)]
    pub app_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFmartLimitResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub check_code: Option<FMartLimitCheckCode>,
    #[serde(default)]
    pub price_float_percent: Option<FmartBotRange>,
    #[serde(default)]
    pub add_position_percent: Option<FmartBotRange>,
    #[serde(default)]
    pub add_position_num: Option<FmartBotRange>,
    #[serde(default)]
    pub init_margin: Option<FmartBotRange>,
    #[serde(default)]
    pub round_tp_percent: Option<FmartBotRange>,
    #[serde(default)]
    pub sl_percent: Option<FmartBotRange>,
    #[serde(default)]
    pub entry_price: Option<FmartBotRange>,
    #[serde(default)]
    pub leverage: Option<FmartBotRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMartLimitCheckCode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FmartBotRange {
    #[serde(default)]
    pub min: Option<String>,
    #[serde(default)]
    pub max: Option<String>,
}
