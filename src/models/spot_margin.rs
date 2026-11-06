use serde::{Deserialize, Serialize};

use crate::models::crypto_loan::CollateralRatioTier;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoricalInterestRateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<InterestRateRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_borrow_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionTiersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<PositionTierItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionTierItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_tiers_ratio_list: Option<Vec<PositionTierRatio>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionTierRatio {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_limit: Option<String>,
    #[serde(rename = "positionMMR", skip_serializing_if = "Option::is_none")]
    pub position_mmr: Option<String>,
    #[serde(rename = "positionIMR", skip_serializing_if = "Option::is_none")]
    pub position_imr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTieredCollateralRatioResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<CollateralTierItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralTierItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_ratio_list: Option<Vec<CollateralRatioTier>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVipMarginDataResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_coin_list: Option<Vec<VipCoinGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipCoinGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<VipCoinDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipCoinDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrowable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_borrow_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_collateral: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_borrowing_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotMarginSetLeverageParams {
    pub leverage: String,
    #[serde(default)]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotMarginSwitchModeParams {
    pub spot_margin_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotMarginSwitchModeResponse {
    #[serde(default)]
    pub spot_margin_mode: Option<String>,
}

pub type GetSpotMarginTradeCoinStateResponse = serde_json::Value;
pub type GetSpotMarginTradeFlexibleAvailableInventoryResponse = serde_json::Value;
pub type GetSpotMarginTradeAutoRepayModeResponse = serde_json::Value;
pub type GetSpotMarginTradeMaxBorrowableResponse = serde_json::Value;
pub type GetSpotMarginTradeRepaymentAvailableAmountResponse = serde_json::Value;
pub type GetSpotMarginTradeStateResponse = serde_json::Value;
pub type QueryFixedAvailableInventoryResponse = FixedAvailableInventoryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedAvailableInventoryResult {
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub term: Option<String>,
    #[serde(default)]
    pub annual_rate: Option<String>,
    #[serde(default)]
    pub available_inventory: Option<String>,
    #[serde(default)]
    pub update_time: Option<String>,
}

pub type QueryFixedBorrowContractsResponse = serde_json::Value;
pub type QueryFixedBorrowOrdersResponse = serde_json::Value;
pub type QueryFixedBorrowMarketResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenewFixedBorrowParams {
    pub loan_id: String,
    #[serde(default)]
    pub qty: Option<String>,
}

pub type RenewFixedBorrowResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountFixedBorrowParams {
    pub order_currency: String,
    pub order_amount: String,
    pub annual_rate: String,
    pub term: String,
    #[serde(default)]
    pub repay_type: Option<String>,
    #[serde(default)]
    pub strategy_type: Option<String>,
}

pub type AccountFixedBorrowResponse = serde_json::Value;
pub type QueryBorrowLiabilityResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoRepayModeParams {
    #[serde(default)]
    pub currency: Option<String>,
    pub auto_repay_mode: String,
}

pub type SetAutoRepayModeResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryFixedBorrowPenaltyInterestResponse {
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
pub struct CancelFixedBorrowOrderPostParams {
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelFixedBorrowOrderPostResponse {
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
