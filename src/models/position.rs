//! Position models.

use crate::models::common::*;
use serde::{Deserialize, Serialize};

/// Position list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionList {
    /// Category
    pub category: String,
    /// List of positions
    pub list: Vec<Position>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Position info.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Position index
    pub position_idx: i32,
    /// Risk ID
    #[serde(default)]
    pub risk_id: i32,
    /// Risk limit value
    #[serde(default)]
    pub risk_limit_value: String,
    /// Symbol
    pub symbol: String,
    /// Side
    pub side: String,
    /// Size
    pub size: String,
    /// Average entry price
    #[serde(default)]
    pub avg_price: String,
    /// Position value
    #[serde(default)]
    pub position_value: String,
    /// Trade mode (0=cross, 1=isolated)
    #[serde(default)]
    pub trade_mode: i32,
    /// Position status
    #[serde(default)]
    pub position_status: String,
    /// Leverage
    #[serde(default)]
    pub leverage: String,
    /// Mark price
    #[serde(default)]
    pub mark_price: String,
    /// Liquidation price
    #[serde(default)]
    pub liq_price: String,
    /// Bust price
    #[serde(default)]
    pub bust_price: String,
    /// Position margin
    #[serde(default)]
    pub position_mm: String,
    /// Position initial margin
    #[serde(default)]
    pub position_im: String,
    /// Take profit price
    #[serde(default)]
    pub take_profit: String,
    /// Stop loss price
    #[serde(default)]
    pub stop_loss: String,
    /// Trailing stop
    #[serde(default)]
    pub trailing_stop: String,
    /// Unrealised PnL
    #[serde(default)]
    pub unrealised_pnl: String,
    /// Cumulative realised PnL
    #[serde(default)]
    pub cum_realised_pnl: String,
    /// Created time
    #[serde(default)]
    pub created_time: String,
    /// Updated time
    #[serde(default)]
    pub updated_time: String,
}

/// Set leverage request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageParams {
    /// Category
    pub category: Category,
    /// Symbol
    pub symbol: String,
    /// Buy leverage
    pub buy_leverage: String,
    /// Sell leverage
    pub sell_leverage: String,
}

/// Trading stop request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingStopParams {
    /// Category
    pub category: Category,
    /// Symbol
    pub symbol: String,
    /// Take profit price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// Stop loss price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// Trailing stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_stop: Option<String>,
    /// Take profit trigger
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// Stop loss trigger
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Position index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
}

/// Switch position mode request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPositionModeParams {
    /// Category
    pub category: Category,
    /// Symbol (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Coin (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Mode (0=merged, 3=both sides)
    pub mode: i32,
}

/// Set risk limit request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitParams {
    /// Category
    pub category: Category,
    /// Symbol
    pub symbol: String,
    /// Risk ID
    pub risk_id: i32,
    /// Position index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
}

/// Add margin request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarginParams {
    /// Category
    pub category: Category,
    /// Symbol
    pub symbol: String,
    /// Margin amount
    pub margin: String,
    /// Position index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
}

/// Closed PnL response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlList {
    /// Category
    pub category: String,
    /// List of closed PnL records
    pub list: Vec<ClosedPnl>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Closed PnL record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnl {
    /// Symbol
    pub symbol: String,
    /// Order ID
    pub order_id: String,
    /// Side
    pub side: String,
    /// Qty
    pub qty: String,
    /// Order price
    pub order_price: String,
    /// Order type
    pub order_type: String,
    /// Exec type
    pub exec_type: String,
    /// Closed size
    pub closed_size: String,
    /// Cumulative entry value
    pub cum_entry_value: String,
    /// Average entry price
    pub avg_entry_price: String,
    /// Cumulative exit value
    pub cum_exit_value: String,
    /// Average exit price
    pub avg_exit_price: String,
    /// Closed PnL
    pub closed_pnl: String,
    /// Fill count
    pub fill_count: String,
    /// Leverage
    pub leverage: String,
    /// Created time
    pub created_time: String,
    /// Updated time
    pub updated_time: String,
}

/// Execution list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionList {
    /// Category
    pub category: String,
    /// List of executions
    pub list: Vec<Execution>,
    /// Next page cursor
    #[serde(default)]
    pub next_page_cursor: String,
}

/// Execution record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    /// Symbol
    pub symbol: String,
    /// Order ID
    pub order_id: String,
    /// Order link ID
    #[serde(default)]
    pub order_link_id: String,
    /// Side
    pub side: String,
    /// Order price
    pub order_price: String,
    /// Order qty
    pub order_qty: String,
    /// Order type
    pub order_type: String,
    /// Exec ID
    pub exec_id: String,
    /// Exec price
    pub exec_price: String,
    /// Exec qty
    pub exec_qty: String,
    /// Exec fee
    pub exec_fee: String,
    /// Exec type
    pub exec_type: String,
    /// Exec value
    pub exec_value: String,
    /// Fee rate
    #[serde(default)]
    pub fee_rate: String,
    /// Exec time
    pub exec_time: String,
}

pub type AddReduceMarginResponse = AddReduceMarginResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddReduceMarginResult {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub position_idx: i32,
    #[serde(default)]
    pub risk_id: i32,
    #[serde(default)]
    pub risk_limit_value: String,
    #[serde(default)]
    pub size: String,
    #[serde(default)]
    pub avg_price: String,
    #[serde(default)]
    pub liq_price: String,
    #[serde(default)]
    pub bust_price: String,
    #[serde(default)]
    pub mark_price: String,
    #[serde(default)]
    pub position_value: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub auto_add_margin: i32,
    #[serde(default)]
    pub position_status: String,
    #[serde(rename = "positionIM", default)]
    pub position_im: String,
    #[serde(rename = "positionMM", default)]
    pub position_mm: String,
    #[serde(default)]
    pub take_profit: String,
    #[serde(default)]
    pub stop_loss: String,
    #[serde(default)]
    pub trailing_stop: String,
    #[serde(default)]
    pub unrealised_pnl: String,
    #[serde(default)]
    pub cum_realised_pnl: String,
    #[serde(default)]
    pub created_time: String,
    #[serde(default)]
    pub updated_time: String,
}

pub type ConfirmNewRiskLimitResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmNewRiskLimitParams {
    pub category: String,
    pub symbol: String,
}

pub type GetClosePositionResponse = GetClosePositionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClosePositionResult {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub list: Vec<GetClosePositionItem>,
    #[serde(default)]
    pub next_page_cursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClosePositionItem {
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub qty: String,
    #[serde(default)]
    pub avg_entry_price: String,
    #[serde(default)]
    pub avg_exit_price: String,
    #[serde(default)]
    pub delivery_price: String,
    #[serde(default)]
    pub total_open_fee: String,
    #[serde(default)]
    pub total_close_fee: String,
    #[serde(default)]
    pub delivery_fee: String,
    #[serde(default)]
    pub total_pnl: String,
    #[serde(default)]
    pub open_time: i64,
    #[serde(default)]
    pub close_time: i64,
}

pub type GetClosedPnlResponse = GetClosedPnlResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClosedPnlResult {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub list: Vec<ClosedPnlItem>,
    #[serde(default)]
    pub next_page_cursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlItem {
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub qty: String,
    #[serde(default)]
    pub order_price: String,
    #[serde(default)]
    pub order_type: String,
    #[serde(default)]
    pub exec_type: String,
    #[serde(default)]
    pub closed_size: String,
    #[serde(default)]
    pub cum_entry_value: String,
    #[serde(default)]
    pub avg_entry_price: String,
    #[serde(default)]
    pub cum_exit_value: String,
    #[serde(default)]
    pub avg_exit_price: String,
    #[serde(default)]
    pub closed_pnl: String,
    #[serde(default)]
    pub fill_count: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub open_fee: String,
    #[serde(default)]
    pub close_fee: String,
    #[serde(default)]
    pub created_time: String,
    #[serde(default)]
    pub updated_time: String,
}

pub type GetMovePositionHistoryResponse = GetMovePositionHistoryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMovePositionHistoryResult {
    #[serde(default)]
    pub list: Vec<GetMovePositionHistoryItem>,
    #[serde(default)]
    pub next_page_cursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMovePositionHistoryItem {
    #[serde(default)]
    pub block_trade_id: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub user_id: i64,
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub qty: String,
    #[serde(default)]
    pub exec_fee: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub exec_id: String,
    #[serde(default)]
    pub result_code: i32,
    #[serde(default)]
    pub result_message: String,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
    #[serde(default)]
    pub reject_party: String,
}

pub type GetPositionInfoResponse = GetPositionInfoResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionInfoResult {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub list: Vec<PositionInfo>,
    #[serde(default)]
    pub next_page_cursor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    #[serde(default)]
    pub position_idx: i32,
    #[serde(default)]
    pub risk_id: i32,
    #[serde(default)]
    pub risk_limit_value: String,
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub size: String,
    #[serde(default)]
    pub avg_price: String,
    #[serde(default)]
    pub position_value: String,
    #[serde(default)]
    pub trade_mode: i32,
    #[serde(default)]
    pub auto_add_margin: i32,
    #[serde(default)]
    pub position_status: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub mark_price: String,
    #[serde(default)]
    pub liq_price: String,
    #[serde(default)]
    pub bust_price: String,
    #[serde(rename = "positionIM", default)]
    pub position_im: String,
    #[serde(rename = "positionMM", default)]
    pub position_mm: String,
    #[serde(default)]
    pub position_balance: String,
    #[serde(default)]
    pub tpsl_mode: String,
    #[serde(default)]
    pub take_profit: String,
    #[serde(default)]
    pub stop_loss: String,
    #[serde(default)]
    pub trailing_stop: String,
    #[serde(default)]
    pub unrealised_pnl: String,
    #[serde(default)]
    pub cur_realised_pnl: String,
    #[serde(default)]
    pub cum_realised_pnl: String,
    #[serde(default)]
    pub break_even_price: String,
    #[serde(default)]
    pub adl_rank_indicator: i32,
    #[serde(default)]
    pub is_reduce_only: bool,
    #[serde(default)]
    pub mmr_sys_updated_time: String,
    #[serde(default)]
    pub leverage_sys_updated_time: String,
    #[serde(rename = "positionIMByMp", default)]
    pub position_im_by_mp: String,
    #[serde(rename = "positionMMByMp", default)]
    pub position_mm_by_mp: String,
    #[serde(default)]
    pub session_avg_price: String,
    #[serde(default)]
    pub delta: String,
    #[serde(default)]
    pub gamma: String,
    #[serde(default)]
    pub vega: String,
    #[serde(default)]
    pub theta: String,
    #[serde(default)]
    pub seq: i64,
    #[serde(default)]
    pub created_time: String,
    #[serde(default)]
    pub updated_time: String,
}

pub type MovePositionResponse = MovePositionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionResult {
    #[serde(default)]
    pub block_trade_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub reject_party: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionLeg {
    pub category: String,
    pub symbol: String,
    pub price: String,
    pub side: String,
    pub qty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionParams {
    pub from_uid: String,
    pub to_uid: String,
    pub list: Vec<MovePositionLeg>,
}

pub type SetAutoAddMarginResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoAddMarginParams {
    pub category: String,
    pub symbol: String,
    pub auto_add_margin: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
}

pub type SetLeverageResponse = serde_json::Value;

pub type SwitchPositionModeResponse = serde_json::Value;

pub type GetPositionSymbolInfoResponse = serde_json::Value;
