use crate::client::BybitClient;
use crate::error::Result;
use crate::models::grid::*;

impl BybitClient {
    pub async fn close_grid_bot(&self, params: CloseGridBotParams) -> Result<CloseGridBotResponse> {
        self.post("/v5/grid/close-grid", &params).await
    }

    pub async fn create_grid_bot(
        &self,
        params: CreateGridBotParams,
    ) -> Result<CreateGridBotResponse> {
        self.post("/v5/grid/create-grid", &params).await
    }

    pub async fn query_grid_detail(
        &self,
        params: QueryGridDetailParams,
    ) -> Result<QueryGridDetailResponse> {
        self.post("/v5/grid/query-grid-detail", &params).await
    }

    pub async fn validate_grid_input(
        &self,
        params: ValidateGridInputParams,
    ) -> Result<ValidateGridInputResponse> {
        self.post("/v5/grid/validate-input", &params).await
    }
}
