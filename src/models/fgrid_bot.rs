use crate::models::AccountType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFgridDetailParams {
    pub bot_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFgridDetailResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub detail: Option<FutureGridDetail>,
    #[serde(default)]
    pub debug_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureGridDetail {
    #[serde(default)]
    pub bot_id: Option<i64>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub status: Option<FutureGridStatus>,
    #[serde(default)]
    pub base_token: Option<String>,
    #[serde(default)]
    pub quote_token: Option<String>,
    #[serde(default)]
    pub min_price: Option<String>,
    #[serde(default)]
    pub max_price: Option<String>,
    #[serde(default)]
    pub cell_number: Option<i64>,
    #[serde(default)]
    pub grid_type: Option<FutureGridType>,
    #[serde(default)]
    pub grid_mode: Option<FutureGridMode>,
    #[serde(default)]
    pub stop_loss_per: Option<String>,
    #[serde(default)]
    pub take_profit_per: Option<String>,
    #[serde(default)]
    pub real_leverage: Option<String>,
    #[serde(default)]
    pub total_investment: Option<String>,
    #[serde(default)]
    pub total_value: Option<String>,
    #[serde(default)]
    pub current_position: Option<String>,
    #[serde(default)]
    pub arbitrage_num: Option<i64>,
    #[serde(default)]
    pub arbitrage_num_24: Option<i64>,
    #[serde(default)]
    pub pnl: Option<String>,
    #[serde(default)]
    pub pnl_per: Option<String>,
    #[serde(default)]
    pub grid_profit: Option<String>,
    #[serde(default)]
    pub grid_apr: Option<String>,
    #[serde(default)]
    pub total_apr: Option<String>,
    #[serde(default)]
    pub last_price: Option<String>,
    #[serde(default)]
    pub liquidation_price: Option<String>,
    #[serde(default)]
    pub realised_pnl: Option<String>,
    #[serde(default)]
    pub unrealised_pnl: Option<String>,
    #[serde(default)]
    pub funding_fee: Option<String>,
    #[serde(default)]
    pub position_balance: Option<String>,
    #[serde(default)]
    pub available_balance: Option<String>,
    #[serde(default)]
    pub total_order_balance: Option<String>,
    #[serde(default)]
    pub close_reason: Option<FGridBotStopType>,
    #[serde(default)]
    pub allow_follow: Option<i64>,
    #[serde(default)]
    pub operation_time: Option<i64>,
    #[serde(default)]
    pub create_time: Option<i64>,
    #[serde(default)]
    pub modify_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub min_profit: Option<String>,
    #[serde(default)]
    pub max_profit: Option<String>,
    #[serde(default)]
    pub leverage: Option<String>,
    #[serde(default)]
    pub mark_price: Option<String>,
    #[serde(default)]
    pub bot_close_code: Option<FgridBotBotCloseCode>,
    #[serde(default)]
    pub futures_pos_side: Option<FuturesPositionSide>,
    #[serde(default)]
    pub follow_num: Option<i64>,
    #[serde(default)]
    pub entry_price: Option<String>,
    #[serde(default)]
    pub stop_loss_price: Option<String>,
    #[serde(default)]
    pub take_profit_price: Option<String>,
    #[serde(default)]
    pub tp_sl_type: Option<TpSlType>,
    #[serde(default)]
    pub real_close_price: Option<String>,
    #[serde(default)]
    pub min_price_precision: Option<String>,
    #[serde(default)]
    pub tick_size: Option<String>,
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
    pub copy_trade_is_master: Option<bool>,
    #[serde(default)]
    pub copy_trade_follower_num: Option<String>,
    #[serde(default)]
    pub init_bonus: Option<String>,
    #[serde(default)]
    pub used_bonus_amount: Option<String>,
    #[serde(default)]
    pub used_reward_id: Option<String>,
    #[serde(default)]
    pub taker_fee_rate: Option<String>,
    #[serde(default)]
    pub maker_fee_rate: Option<String>,
    #[serde(default)]
    pub equity: Option<String>,
    #[serde(default)]
    pub trailing_stop_exit_equity: Option<String>,
    #[serde(default)]
    pub trailing_stop_per: Option<String>,
    #[serde(default)]
    pub i_m_rate: Option<String>,
    #[serde(default)]
    pub m_m_rate: Option<String>,
    #[serde(default)]
    pub move_up_price: Option<String>,
    #[serde(default)]
    pub move_down_price: Option<String>,
    #[serde(default)]
    pub curr_min_price: Option<String>,
    #[serde(default)]
    pub curr_max_price: Option<String>,
    #[serde(default)]
    pub adl_rank_indicator: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureGridStatus {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureGridType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureGridMode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FGridBotStopType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FgridBotBotCloseCode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesPositionSide {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TpSlType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotAccountType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseFgridBotParams {
    pub bot_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseFgridBotResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub bot_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFgridBotParams {
    pub symbol: String,
    pub grid_mode: FutureGridModeFgridBotSingleCreate,
    pub min_price: String,
    pub max_price: String,
    pub cell_number: i64,
    pub leverage: String,
    pub grid_type: FutureGridTypeFgridBotSingleCreate,
    pub total_investment: String,
    #[serde(default)]
    pub take_profit_per: Option<String>,
    #[serde(default)]
    pub stop_loss_per: Option<String>,
    #[serde(default)]
    pub entry_price: Option<String>,
    #[serde(default)]
    pub source: Option<FGridSource>,
    #[serde(default)]
    pub followed_grid_id: Option<i64>,
    #[serde(default)]
    pub tools_discovery_parameter: Option<ToolsDiscoveryParameter>,
    #[serde(default)]
    pub stop_loss_price: Option<String>,
    #[serde(default)]
    pub take_profit_price: Option<String>,
    #[serde(default)]
    pub tp_sl_type: Option<TpSlTypeFgridBotSingleCreate>,
    #[serde(default)]
    pub block_source: Option<FgridBotBlockSource>,
    #[serde(default)]
    pub create_type: Option<FgridBotCreateType>,
    #[serde(default)]
    pub init_bonus: Option<String>,
    #[serde(default)]
    pub business_remark: Option<String>,
    #[serde(default)]
    pub trailing_stop_per: Option<String>,
    #[serde(default)]
    pub move_up_price: Option<String>,
    #[serde(default)]
    pub move_down_price: Option<String>,
    #[serde(default)]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureGridModeFgridBotSingleCreate {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureGridTypeFgridBotSingleCreate {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FGridSource {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolsDiscoveryParameter {
    #[serde(default)]
    pub strategy_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TpSlTypeFgridBotSingleCreate {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FgridBotBlockSource {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FgridBotCreateType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFgridBotResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub bot_id: Option<i64>,
    #[serde(default)]
    pub check_code: Option<FgridCheckCode>,
    #[serde(default)]
    pub ban_reason_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FgridCheckCode {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateFgridInputParams {
    pub symbol: String,
    pub cell_number: i64,
    pub min_price: String,
    pub max_price: String,
    pub leverage: String,
    pub grid_type: FutureGridTypeFgridBotValidateInput,
    pub grid_mode: FutureGridModeFgridBotValidateInput,
    #[serde(default)]
    pub stop_loss_price: Option<String>,
    #[serde(default)]
    pub take_profit_price: Option<String>,
    #[serde(default)]
    pub tp_sl_type: Option<TpSlTypeFgridBotValidateInput>,
    #[serde(default)]
    pub entry_price: Option<String>,
    #[serde(default)]
    pub stop_loss_per: Option<String>,
    #[serde(default)]
    pub take_profit_per: Option<String>,
    #[serde(default)]
    pub trailing_stop_per: Option<String>,
    #[serde(default)]
    pub init_margin: Option<String>,
    #[serde(default)]
    pub move_up_price: Option<String>,
    #[serde(default)]
    pub move_down_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureGridTypeFgridBotValidateInput {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureGridModeFgridBotValidateInput {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TpSlTypeFgridBotValidateInput {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateFgridInputResponse {
    #[serde(default)]
    pub status_code: Option<i64>,
    #[serde(default)]
    pub debug_msg: Option<String>,
    #[serde(default)]
    pub investment: Option<FgridBotRange>,
    #[serde(default)]
    pub profit: Option<FgridBotRange>,
    #[serde(default)]
    pub cell_number: Option<FgridBotRange>,
    #[serde(default)]
    pub min_price: Option<FgridBotRange>,
    #[serde(default)]
    pub max_price: Option<FgridBotRange>,
    #[serde(default)]
    pub leverage: Option<FgridBotRange>,
    #[serde(default)]
    pub stop_loss: Option<FgridBotRange>,
    #[serde(default)]
    pub take_profit: Option<FgridBotRange>,
    #[serde(default)]
    pub check_code: Option<FgridCheckCode>,
    #[serde(default)]
    pub take_profit_price: Option<FgridBotRange>,
    #[serde(default)]
    pub stop_loss_price: Option<FgridBotRange>,
    #[serde(default)]
    pub entry_price: Option<FgridBotRange>,
    #[serde(default)]
    pub cell_gtv0_per: Option<String>,
    #[serde(default)]
    pub trailing_stop_per: Option<FgridBotRange>,
    #[serde(default)]
    pub long_liq_price: Option<String>,
    #[serde(default)]
    pub short_liq_price: Option<String>,
    #[serde(default)]
    pub move_up_price: Option<FgridBotRange>,
    #[serde(default)]
    pub move_down_price: Option<FgridBotRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FgridBotRange {
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<String>,
}
