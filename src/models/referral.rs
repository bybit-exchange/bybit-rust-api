use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReferralCampaignRewardsParams {}

pub type GetReferralCampaignRewardsResponse = Option<CampaignResult>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignResult {
    #[serde(default)]
    pub rebate_rule: Option<ReferralCampaignRebateRatioRules>,
    #[serde(default)]
    pub tasks: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralCampaignRebateRatioRules {
    #[serde(default)]
    pub deposit_amount: Option<i64>,
    #[serde(default)]
    pub trade_amount: Option<i64>,
    #[serde(default)]
    pub rebate_ratio_rules: Option<Vec<ReferralCampaignRebateRatioRule>>,
    #[serde(default)]
    pub rebate_duration_days: Option<i64>,
    #[serde(default)]
    pub tradefi_deposit_amount: Option<i64>,
    #[serde(default)]
    pub tradefi_trade_amount: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralCampaignRebateRatioRule {
    #[serde(default)]
    pub rebate_ratio: Option<i64>,
    #[serde(default)]
    pub referral_count: Option<i64>,
    #[serde(default)]
    pub has_trade_condition: Option<bool>,
    #[serde(default)]
    pub trade_amount_condition: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReferralCommissionRecordsParams {
    #[serde(default)]
    pub user_id_list: Option<Vec<i64>>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub product_line: Option<BusinessUnit>,
}

pub type BusinessUnit = crate::models::OpenApiScalar;

pub type GetReferralCommissionRecordsResponse = Option<GetReferralCommissionRecordsResult>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReferralCommissionRecordsResult {
    #[serde(default)]
    pub records: Option<Vec<ReferralCommissionRecord>>,
    #[serde(default)]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralCommissionRecord {
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub payment_time: Option<i64>,
    #[serde(default)]
    pub rate: Option<String>,
    #[serde(default)]
    pub business_unit: Option<BusinessUnit>,
    #[serde(default)]
    pub commission_origin_coin: Option<String>,
    #[serde(default)]
    pub commission_origin_amount: Option<String>,
    #[serde(default)]
    pub commission_usdt_amount: Option<String>,
    #[serde(default)]
    pub commission_status: Option<CommissionStatus>,
    #[serde(default)]
    pub is_boost_m2: Option<bool>,
}

pub type CommissionStatus = crate::models::OpenApiScalar;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReferralUserIdentityProfileParams {}

pub type GetReferralUserIdentityProfileResponse = GetReferralUserIdentityProfileResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReferralUserIdentityProfileResult {
    #[serde(default)]
    pub is_affiliate: Option<bool>,
    #[serde(default)]
    pub is_m1: Option<bool>,
    #[serde(default)]
    pub inviter_affiliate_id: Option<i64>,
    #[serde(default)]
    pub inviter_affiliate_user_id: Option<i64>,
    #[serde(default)]
    pub inviter_user_id: Option<i64>,
    #[serde(default)]
    pub commission_info: Option<ReferralIdentityCommissionInfoResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralIdentityCommissionInfoResult {
    #[serde(default)]
    pub total_commission: Option<String>,
    #[serde(default)]
    pub yesterday_commission: Option<String>,
    #[serde(default)]
    pub total_friends: Option<i64>,
    #[serde(default)]
    pub qualified_friends: Option<i64>,
    #[serde(default)]
    pub wallet_balance: Option<String>,
    #[serde(default)]
    pub qualified_friends_quarter_total_trade_volume: Option<String>,
    #[serde(default)]
    pub cur_quarter_active_friends_count: Option<i64>,
    #[serde(default)]
    pub has_old_referral: Option<i64>,
    #[serde(default)]
    pub degrade_type: Option<i64>,
    #[serde(default)]
    pub commission_rate: Option<String>,
    #[serde(default)]
    pub upgrade_unified_account: Option<i64>,
    #[serde(default)]
    pub withdrawal_switch: Option<i64>,
    #[serde(default)]
    pub last_quarter_qualified_friends_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReferralUserInvitationsParams {
    #[serde(default)]
    pub user_id_list: Option<Vec<i64>>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub friend_status: Option<FriendStatus>,
}

pub type FriendStatus = crate::models::OpenApiScalar;

pub type GetReferralUserInvitationsResponse = Option<GetReferralUserInvitationsResult>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReferralUserInvitationsResult {
    #[serde(default)]
    pub records: Option<Vec<ReferralUserInvitationRecord>>,
    #[serde(default)]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralUserInvitationRecord {
    #[serde(default)]
    pub user_id_mask: Option<String>,
    #[serde(default)]
    pub sign_time: Option<i64>,
    #[serde(default)]
    pub user_status: Option<FriendStatus>,
    #[serde(default)]
    pub total_commission: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub task_progress: Option<String>,
    #[serde(default)]
    pub not_show_commission: Option<bool>,
    #[serde(default)]
    pub total_base_rewards: Option<String>,
}
