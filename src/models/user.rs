use serde::{Deserialize, Serialize};

use crate::models::common::RedactedString;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateSubItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_aff_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissions_vol: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissions_for_usdt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub became_aff_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateSubListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AffiliateSubItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

pub type GetAffiliateSubListResponse = AffiliateSubListResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateUserItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_kyc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradfi_trade_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissions_30_day: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_amount_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_amount_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_vol_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_vol_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_vol_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradfi_trade_vol_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissions_365_day: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_vol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_vol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_vol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradfi_trade_vol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissions_vol: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateUserListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AffiliateUserItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

pub type GetAffiliateUserListResponse = AffiliateUserListResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateUserInfoResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_amount_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_vol_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_vol_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_vol_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_amount_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_wallet_balance: Option<String>,
    #[serde(rename = "KycLevel", skip_serializing_if = "Option::is_none")]
    pub kyc_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissions_30_day: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissions_365_day: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vol_update_time: Option<String>,
}

pub type GetAffiliateUserInfoResponse = AffiliateUserInfoResult;

pub type GetAffiliateCustomOpenInfoV5Response = AffiliateCustomOpenInfoResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateCustomOpenInfoResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_amount_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_vol_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker_vol_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_vol_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_amount_365_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_wallet_balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vol_update_time: Option<String>,
    #[serde(rename = "KycLevel", skip_serializing_if = "Option::is_none")]
    pub kyc_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradfi_trade_vol_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradfi_trade_vol_365_day: Option<String>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissions_30_day: Option<serde_json::Value>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commissions_365_day: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_send_amount_30_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_ftt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_ftt: Option<String>,
}

pub type ErrorResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubApiKeyParams {
    pub subuid: i64,
    pub read_only: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<String>,
    pub permissions: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

pub type CreateSubApiKeyResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubMemberParams {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<RedactedString>,
    pub member_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_uta: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

pub type CreateSubMemberResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteApiKeyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apikey: Option<String>,
}

pub type DeleteApiKeyResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubApiKeyParams {
    pub subuid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apikey: Option<String>,
}

pub type DeleteSubApiKeyResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubMemberV5Params {
    pub subuid: i64,
}

pub type DeleteSubMemberV5Response = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrozenSubMemberParams {
    pub subuid: i64,
    pub frozen: i32,
}

pub type FrozenSubMemberResponse = serde_json::Value;

pub type ListSubApiKeysV5Response = serde_json::Value;

pub type QueryApiKeyResponse = serde_json::Value;

pub type QueryEscrowSubMembersV5Response = serde_json::Value;

pub type QueryReferralsResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitee_uid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

pub type QuerySubMembersV5Response = serde_json::Value;

pub type QuerySubMembersResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignAgreementParams {
    pub category: i32,
    pub agree: bool,
}

pub type SignAgreementResponse = serde_json::Value;

pub type UpdateAPIKeyResponse = serde_json::Value;

pub type UpdateSubAPIKeyResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApiKeyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApiKeyResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub read_only: i32,
    /// API secret. Wrapped in [`RedactedString`] so it never appears in
    /// `Debug` / log output.
    #[serde(default)]
    pub secret: RedactedString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
    #[serde(default)]
    pub ips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubApiKeyParams {
    pub subuid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apikey: Option<String>,
    pub read_only: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubApiKeyResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub read_only: i32,
    /// API secret. Wrapped in [`RedactedString`] so it never appears in
    /// `Debug` / log output.
    #[serde(default)]
    pub secret: RedactedString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
    #[serde(default)]
    pub ips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyPermissions {
    #[serde(rename = "ContractTrade", skip_serializing_if = "Option::is_none")]
    pub contract_trade: Option<Vec<String>>,
    #[serde(rename = "Spot", skip_serializing_if = "Option::is_none")]
    pub spot: Option<Vec<String>>,
    #[serde(rename = "Wallet", skip_serializing_if = "Option::is_none")]
    pub wallet: Option<Vec<String>>,
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "Derivatives", skip_serializing_if = "Option::is_none")]
    pub derivatives: Option<Vec<String>>,
    #[serde(rename = "CopyTrading", skip_serializing_if = "Option::is_none")]
    pub copy_trading: Option<Vec<String>>,
    #[serde(rename = "BlockTrade", skip_serializing_if = "Option::is_none")]
    pub block_trade: Option<Vec<String>>,
    #[serde(rename = "Exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<Vec<String>>,
    #[serde(rename = "NFT", skip_serializing_if = "Option::is_none")]
    pub nft: Option<Vec<String>>,
    #[serde(rename = "Affiliate", skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<Vec<String>>,
}

pub type QueryReferralCodeResponse = QueryReferralCodeResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryReferralCodeResult {
    #[serde(default)]
    pub referral_codes: Option<Vec<ReferralCodeItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralCodeItem {
    pub referral_code: String,
    pub referral_link: String,
    pub scene: i64,
}
