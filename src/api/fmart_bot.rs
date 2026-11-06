//! F-Martingale Bot API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::fmart_bot::*;

impl BybitClient {
    /// Create a new futures Martingale bot with DCA averaging strategy.
    pub async fn create_f_mart_bot(
        &self,
        params: CreateFMartBotParams,
    ) -> Result<CreateFMartBotResponse> {
        self.post("/v5/fmartingalebot/create", &params).await
    }
}

impl BybitClient {
    pub async fn close_fmart_bot(
        &self,
        params: CloseFmartBotParams,
    ) -> Result<CloseFmartBotResponse> {
        self.post("/v5/fmartingalebot/close", &params).await
    }

    pub async fn get_fmart_detail(
        &self,
        params: GetFmartDetailParams,
    ) -> Result<GetFmartDetailResponse> {
        self.post("/v5/fmartingalebot/detail", &params).await
    }

    pub async fn get_fmart_limit(
        &self,
        params: GetFmartLimitParams,
    ) -> Result<GetFmartLimitResponse> {
        self.post("/v5/fmartingalebot/getlimit", &params).await
    }
}
