use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsLoanAssociationUidParams {
    pub uid: String,
    pub operate: String,
}

pub type InsLoanAssociationUidResponse = serde_json::Value;
pub type InsLoanCoinDeltaAmountResponse = serde_json::Value;
pub type InsLoanDelayLiqStatusResponse = DelayLiqStatusResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelayLiqStatusResult {
    #[serde(default)]
    pub ltv: Option<String>,
    #[serde(default)]
    pub liquidation_status: Option<i64>,
    #[serde(default)]
    pub delay_liq_start_time: Option<String>,
    #[serde(default)]
    pub delay_liq_remaining_sec: Option<String>,
}

pub type InsLoanEnsureTokensConvertResponse = serde_json::Value;
pub type InsLoanEnsureTokensResponse = serde_json::Value;
pub type InsLoanGetLoanOrderResponse = serde_json::Value;
pub type InsLoanLtvConvertResponse = serde_json::Value;
pub type InsLoanProductInfosResponse = serde_json::Value;
pub type InsLoanRepaidHistoryResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsLoanRepayLoanParams {
    pub token: String,
    pub quantity: String,
}

pub type InsLoanRepayLoanResponse = serde_json::Value;
