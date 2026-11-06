use crate::client::BybitClient;
use crate::error::Result;
use crate::models::combo_bot::*;

impl BybitClient {
    pub async fn close_combo_bot(
        &self,
        params: CloseComboBotParams,
    ) -> Result<CloseComboBotResponse> {
        self.post("/v5/fcombobot/close", &params).await
    }

    pub async fn create_combo_bot(
        &self,
        params: CreateComboBotParams,
    ) -> Result<CreateComboBotResponse> {
        self.post("/v5/fcombobot/create", &params).await
    }

    pub async fn get_combo_detail(
        &self,
        params: GetComboDetailParams,
    ) -> Result<GetComboDetailResponse> {
        self.post("/v5/fcombobot/detail", &params).await
    }

    pub async fn get_combo_limit(
        &self,
        params: GetComboLimitParams,
    ) -> Result<GetComboLimitResponse> {
        self.post("/v5/fcombobot/getlimit", &params).await
    }
}
