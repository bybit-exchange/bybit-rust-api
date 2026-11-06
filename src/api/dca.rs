use crate::client::BybitClient;
use crate::error::Result;
use crate::models::dca::*;

impl BybitClient {
    pub async fn close_dcabot(&self, params: CloseDcabotParams) -> Result<CloseDcabotResponse> {
        self.post("/v5/dca/close-bot", &params).await
    }

    pub async fn create_dcabot(&self, params: CreateDcabotParams) -> Result<CreateDcabotResponse> {
        self.post("/v5/dca/create-bot", &params).await
    }
}
