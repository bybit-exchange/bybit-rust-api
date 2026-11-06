use crate::client::BybitClient;
use crate::error::Result;
use crate::models::spot_x::*;

impl BybitClient {
    pub async fn get_launchpool_project_list(
        &self,
        status: i64,
        activity_coin: Option<&str>,
        project_id: Option<&str>,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<GetLaunchpoolProjectListResponse> {
        let status_string = status.to_string();
        let mut params = vec![("status", status_string.as_str())];
        if let Some(value) = activity_coin {
            params.push(("activityCoin", value));
        }
        if let Some(value) = project_id {
            params.push(("projectId", value));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        self.get_public("/v5/spot-x/launchpool/project/list", &params)
            .await
    }

    pub async fn get_launchpool_user_activity_log(
        &self,
        params: GetLaunchpoolUserActivityLogParams,
    ) -> Result<GetLaunchpoolUserActivityLogResponse> {
        self.post("/v5/spot-x/launchpool/user/activity-log", &params)
            .await
    }

    pub async fn get_launchpool_user_current_staking(
        &self,
    ) -> Result<GetLaunchpoolUserCurrentStakingResponse> {
        let params = vec![];
        self.get("/v5/spot-x/launchpool/user/current-staking", &params)
            .await
    }

    pub async fn get_launchpool_user_history(
        &self,
        params: GetLaunchpoolUserHistoryParams,
    ) -> Result<GetLaunchpoolUserHistoryResponse> {
        self.post("/v5/spot-x/launchpool/user/history", &params)
            .await
    }

    pub async fn get_puzzle_project_list(
        &self,
        status: i64,
        project_id: Option<&str>,
        activity_coin: Option<&str>,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<GetPuzzleProjectListResponse> {
        let status_string = status.to_string();
        let mut params = vec![("status", status_string.as_str())];
        if let Some(value) = project_id {
            params.push(("projectId", value));
        }
        if let Some(value) = activity_coin {
            params.push(("activityCoin", value));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        self.get_public("/v5/spot-x/puzzle/project/list", &params)
            .await
    }

    pub async fn get_token_splash_project_list(
        &self,
        status: i64,
        project_id: Option<&str>,
        activity_coin: Option<&str>,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<GetTokenSplashProjectListResponse> {
        let status_string = status.to_string();
        let mut params = vec![("status", status_string.as_str())];
        if let Some(value) = project_id {
            params.push(("projectId", value));
        }
        if let Some(value) = activity_coin {
            params.push(("activityCoin", value));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        self.get_public("/v5/spot-x/token-splash/project/list", &params)
            .await
    }

    pub async fn get_token_splash_user_activity_params(
        &self,
        project_id: Option<&str>,
        activity_coin: Option<&str>,
    ) -> Result<GetTokenSplashUserActivityParamsResponse> {
        let mut params = vec![];
        if let Some(value) = project_id {
            params.push(("projectId", value));
        }
        if let Some(value) = activity_coin {
            params.push(("activityCoin", value));
        }
        self.get("/v5/spot-x/token-splash/user/activity-params", &params)
            .await
    }
}
