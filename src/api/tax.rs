use crate::client::BybitClient;
use crate::error::Result;
use crate::models::tax::*;

impl BybitClient {
    pub async fn batch_create_tax_reports(
        &self,
        params: BatchCreateTaxReportsParams,
    ) -> Result<BatchCreateTaxReportsResponse> {
        self.post("/v5/fht/compliance/tax/private/batch_create", &params)
            .await
    }

    pub async fn batch_query_tax_reports(
        &self,
        batch_id: &str,
    ) -> Result<BatchQueryTaxReportsResponse> {
        let params = vec![("batchId", batch_id)];
        self.get("/v5/fht/compliance/tax/private/batch_query", &params)
            .await
    }
}
