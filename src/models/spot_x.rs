use serde::{Deserialize, Serialize};

pub type GetLaunchpoolProjectListResponse = Option<ProjectListResult>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectListResult {
    #[serde(default)]
    pub list: Option<Vec<ProjectItemDto>>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectItemDto {
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub activity_coin: Option<String>,
    #[serde(default)]
    pub reward_coin: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub total_reward: Option<String>,
    #[serde(default)]
    pub stake_start_time: Option<String>,
    #[serde(default)]
    pub stake_end_time: Option<String>,
    #[serde(default)]
    pub total_participant_count: Option<String>,
    #[serde(default)]
    pub pools: Option<Vec<PoolInfoDto>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolInfoDto {
    #[serde(default)]
    pub pool_id: Option<String>,
    #[serde(default)]
    pub stake_coin: Option<String>,
    #[serde(default)]
    pub apr: Option<String>,
    #[serde(default)]
    pub total_staked_amount: Option<String>,
    #[serde(default)]
    pub participant_count: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLaunchpoolUserActivityLogParams {
    #[serde(default)]
    pub stake_coin: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub current: Option<i64>,
}

pub type GetLaunchpoolUserActivityLogResponse = Option<ActivityLogRespDto>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityLogRespDto {
    #[serde(default)]
    pub list: Option<Vec<ActivityLogItemDto>>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub current: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityLogItemDto {
    #[serde(default)]
    pub stake_coin: Option<String>,
    #[serde(default)]
    pub reward_coin: Option<String>,
    #[serde(default)]
    pub amount: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub type_: Option<i64>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
}

pub type GetLaunchpoolUserCurrentStakingResponse = Option<CurrentStakingRespDto>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentStakingRespDto {
    #[serde(default)]
    pub total_investment_usd: Option<String>,
    #[serde(default)]
    pub total_earnings_usd: Option<String>,
    #[serde(default)]
    pub today_earnings_usd: Option<String>,
    #[serde(default)]
    pub list: Option<Vec<CurrentStakingItemDto>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentStakingItemDto {
    #[serde(default)]
    pub stake_coin: Option<String>,
    #[serde(default)]
    pub reward_coin: Option<String>,
    #[serde(default)]
    pub stake_amount: Option<String>,
    #[serde(default)]
    pub total_reward: Option<String>,
    #[serde(default)]
    pub auto_redeem_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLaunchpoolUserHistoryParams {
    #[serde(default)]
    pub stake_coin: Option<String>,
    #[serde(default)]
    pub reward_coin: Option<String>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub current: Option<i64>,
}

pub type GetLaunchpoolUserHistoryResponse = Option<UserHistoryRespDto>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserHistoryRespDto {
    #[serde(default)]
    pub list: Option<Vec<HistoryItemDto>>,
    #[serde(default)]
    pub total: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
    #[serde(default)]
    pub current: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryItemDto {
    #[serde(default)]
    pub stake_coin: Option<String>,
    #[serde(default)]
    pub reward_coin: Option<String>,
    #[serde(default)]
    pub total_reward: Option<String>,
    #[serde(default)]
    pub stake_start_time: Option<String>,
    #[serde(default)]
    pub stake_end_time: Option<String>,
}

pub type GetPuzzleProjectListResponse = Option<PuzzleProjectListResult>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PuzzleProjectListResult {
    #[serde(default)]
    pub list: Option<Vec<PuzzleProjectItem>>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PuzzleProjectItem {
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub activity_coin: Option<String>,
    #[serde(default)]
    pub reward_coin: Option<String>,
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub total_reward: Option<String>,
    #[serde(default)]
    pub participant_count: Option<String>,
}

pub type GetTokenSplashProjectListResponse = Option<GetProjectListResult>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectListResult {
    #[serde(default)]
    pub list: Option<Vec<ProjectItemDTO>>,
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectItemDTO {
    #[serde(default)]
    pub status: Option<i64>,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub activity_coin: Option<String>,
    #[serde(default)]
    pub reward_coin: Option<String>,
    #[serde(default)]
    pub total_reward: Option<String>,
    #[serde(default)]
    pub participant_count: Option<String>,
    #[serde(default)]
    pub registration_start_time: Option<String>,
    #[serde(default)]
    pub activity_end_time: Option<String>,
}

pub type GetTokenSplashUserActivityParamsResponse = Option<GetUserActivityParamsResult>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserActivityParamsResult {
    #[serde(default)]
    pub list: Option<Vec<UserActivityItemDTO>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserActivityItemDTO {
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub activity_coin: Option<String>,
    #[serde(default)]
    pub trade_task: Option<TradeTaskDetailDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeTaskDetailDTO {
    #[serde(default)]
    pub trade_required_amount: Option<String>,
    #[serde(default)]
    pub trade_unit: Option<String>,
    #[serde(default)]
    pub trade_token: Option<String>,
    #[serde(default)]
    pub traded_amount: Option<String>,
    #[serde(default)]
    pub max_reward_amount: Option<String>,
    #[serde(default)]
    pub estimated_reward_amount: Option<String>,
    #[serde(default)]
    pub reward_coin: Option<String>,
}
