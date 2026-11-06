use crate::client::BybitClient;
use crate::error::Result;
use crate::models::ins_loan::*;

impl BybitClient {
    pub async fn ins_loan_association_uid(
        &self,
        params: InsLoanAssociationUidParams,
    ) -> Result<InsLoanAssociationUidResponse> {
        self.post("/v5/ins-loan/association-uid", &params).await
    }

    pub async fn ins_loan_coin_delta_amount(
        &self,
        coin: Option<&str>,
    ) -> Result<InsLoanCoinDeltaAmountResponse> {
        let mut params = vec![];
        if let Some(value) = coin {
            params.push(("coin", value));
        }
        self.get("/v5/ins-loan/coin-delta-amount", &params).await
    }

    pub async fn ins_loan_delay_liq_status(&self) -> Result<InsLoanDelayLiqStatusResponse> {
        let params = vec![];
        self.get("/v5/ins-loan/delay-liq-status", &params).await
    }

    pub async fn ins_loan_ensure_tokens_convert(
        &self,
        product_id: Option<&str>,
    ) -> Result<InsLoanEnsureTokensConvertResponse> {
        let mut params = vec![];
        if let Some(value) = product_id {
            params.push(("productId", value));
        }
        self.get("/v5/ins-loan/ensure-tokens-convert", &params)
            .await
    }

    pub async fn ins_loan_ensure_tokens(
        &self,
        product_id: Option<&str>,
    ) -> Result<InsLoanEnsureTokensResponse> {
        let mut params = vec![];
        if let Some(value) = product_id {
            params.push(("productId", value));
        }
        self.get("/v5/ins-loan/ensure-tokens", &params).await
    }

    pub async fn ins_loan_get_loan_order(
        &self,
        order_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
    ) -> Result<InsLoanGetLoanOrderResponse> {
        let mut params = vec![];
        if let Some(value) = order_id {
            params.push(("orderId", value));
        }
        let start_time_string;
        if let Some(value) = start_time {
            start_time_string = value.to_string();
            params.push(("startTime", start_time_string.as_str()));
        }
        let end_time_string;
        if let Some(value) = end_time {
            end_time_string = value.to_string();
            params.push(("endTime", end_time_string.as_str()));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        self.get("/v5/ins-loan/loan-order", &params).await
    }

    pub async fn ins_loan_ltv_convert(&self) -> Result<InsLoanLtvConvertResponse> {
        let params = vec![];
        self.get("/v5/ins-loan/ltv-convert", &params).await
    }

    pub async fn ins_loan_product_infos(
        &self,
        product_id: Option<&str>,
    ) -> Result<InsLoanProductInfosResponse> {
        let mut params = vec![];
        if let Some(value) = product_id {
            params.push(("productId", value));
        }
        self.get("/v5/ins-loan/product-infos", &params).await
    }

    pub async fn ins_loan_repaid_history(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
    ) -> Result<InsLoanRepaidHistoryResponse> {
        let mut params = vec![];
        let start_time_string;
        if let Some(value) = start_time {
            start_time_string = value.to_string();
            params.push(("startTime", start_time_string.as_str()));
        }
        let end_time_string;
        if let Some(value) = end_time {
            end_time_string = value.to_string();
            params.push(("endTime", end_time_string.as_str()));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        self.get("/v5/ins-loan/repaid-history", &params).await
    }

    pub async fn ins_loan_repay_loan(
        &self,
        params: InsLoanRepayLoanParams,
    ) -> Result<InsLoanRepayLoanResponse> {
        self.post("/v5/ins-loan/repay-loan", &params).await
    }
}
