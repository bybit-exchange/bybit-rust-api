use crate::client::BybitClient;
use crate::error::Result;
use crate::models::fgrid_bot::*;

impl BybitClient {
    pub async fn get_fgrid_detail(
        &self,
        params: GetFgridDetailParams,
    ) -> Result<GetFgridDetailResponse> {
        self.post("/v5/fgridbot/detail", &params).await
    }

    pub async fn close_fgrid_bot(
        &self,
        params: CloseFgridBotParams,
    ) -> Result<CloseFgridBotResponse> {
        self.post("/v5/fgridbot/close", &params).await
    }

    pub async fn create_fgrid_bot(
        &self,
        params: CreateFgridBotParams,
    ) -> Result<CreateFgridBotResponse> {
        self.post("/v5/fgridbot/create", &params).await
    }

    pub async fn validate_fgrid_input(
        &self,
        params: ValidateFgridInputParams,
    ) -> Result<ValidateFgridInputResponse> {
        self.post("/v5/fgridbot/validate", &params).await
    }
}
