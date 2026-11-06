//! Crypto Loan API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::crypto_loan::*;

impl BybitClient {
    /// Adjust Collateral (Add or Remove)
    pub async fn adjust_ltv(&self, params: AdjustLtvParams) -> Result<AdjustLtvResponse> {
        self.post("/v5/crypto-loan-common/adjust-ltv", &params)
            .await
    }

    /// Calculate Max Borrowable Amount
    pub async fn calculate_max_loan(
        &self,
        params: CalculateMaxLoanParams,
    ) -> Result<CalculateMaxLoanResponse> {
        self.post("/v5/crypto-loan-common/max-loan", &params).await
    }

    /// Cancel Borrow Order
    pub async fn cancel_fixed_borrow_order(
        &self,
        params: CancelFixedBorrowOrderParams,
    ) -> Result<CancelFixedBorrowOrderResponse> {
        self.post("/v5/crypto-loan-fixed/borrow-order-cancel", &params)
            .await
    }

    /// Create Fixed-Term Borrow Order
    pub async fn create_fixed_borrow(
        &self,
        params: CreateFixedBorrowParams,
    ) -> Result<CreateFixedBorrowResponse> {
        self.post("/v5/crypto-loan-fixed/borrow", &params).await
    }

    /// Fully Repay Loan
    pub async fn fully_repay_fixed_loan(
        &self,
        params: FullyRepayFixedLoanParams,
    ) -> Result<FullyRepayFixedLoanResponse> {
        self.post("/v5/crypto-loan-fixed/fully-repay", &params)
            .await
    }

    /// Get Collateral Adjustment History
    pub async fn get_adjustment_history(
        &self,
        adjust_id: Option<i64>,
        collateral_currency: Option<&str>,
        limit: Option<i64>,
        cursor: Option<i64>,
    ) -> Result<GetAdjustmentHistoryResponse> {
        let adjust_id_str = adjust_id.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());
        let cursor_str = cursor.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(ref s) = adjust_id_str {
            params.push(("adjustId", s.as_str()));
        }
        if let Some(s) = collateral_currency {
            params.push(("collateralCurrency", s));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        if let Some(ref s) = cursor_str {
            params.push(("cursor", s.as_str()));
        }
        self.get("/v5/crypto-loan-common/adjustment-history", &params)
            .await
    }

    /// Get Collateral Currency Data
    pub async fn get_collateral_data(
        &self,
        currency: Option<&str>,
    ) -> Result<GetCollateralDataResponse> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = currency {
            params.push(("currency", s));
        }
        self.get_public("/v5/crypto-loan-common/collateral-data", &params)
            .await
    }

    /// Get Repayment History
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn get_fixed_repayment_history(&self) -> Result<serde_json::Value> {
        self.get("/v5/crypto-loan-fixed/repayment-history", &[])
            .await
    }

    /// Get Supply Contract Info
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn get_fixed_supply_contract_info(&self) -> Result<serde_json::Value> {
        self.get("/v5/crypto-loan-fixed/supply-contract-info", &[])
            .await
    }

    /// Get Supply Order Info
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn get_fixed_supply_order_info(&self) -> Result<serde_json::Value> {
        self.get("/v5/crypto-loan-fixed/supply-order-info", &[])
            .await
    }

    /// Get Supply Market Quotes
    pub async fn get_fixed_supply_order_quote(
        &self,
    ) -> Result<GetCryptoLoanFixedSupplyOrderQuoteResponse> {
        self.get_public("/v5/crypto-loan-fixed/supply-order-quote", &[])
            .await
    }

    /// Get Flexible Borrow History
    pub async fn get_flexible_borrow_history(
        &self,
    ) -> Result<GetCryptoLoanFlexibleBorrowHistoryResponse> {
        self.get("/v5/crypto-loan-flexible/borrow-history", &[])
            .await
    }

    /// Get Ongoing Flexible Borrow Info
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn get_flexible_ongoing_coin(&self) -> Result<serde_json::Value> {
        self.get("/v5/crypto-loan-flexible/ongoing-coin", &[]).await
    }

    /// Get Flexible Repayment History
    pub async fn get_flexible_repayment_history(
        &self,
    ) -> Result<GetCryptoLoanFlexibleRepaymentHistoryResponse> {
        self.get("/v5/crypto-loan-flexible/repayment-history", &[])
            .await
    }

    /// Get Borrow Contract Info
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn get_fixed_borrow_contract_info(
        &self,
        order_id: Option<&str>,
        loan_id: Option<&str>,
        order_currency: Option<&str>,
        term: Option<&str>,
        limit: Option<i64>,
        cursor: Option<i64>,
    ) -> Result<serde_json::Value> {
        let limit_str = limit.map(|v| v.to_string());
        let cursor_str = cursor.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = order_id {
            params.push(("orderId", s));
        }
        if let Some(s) = loan_id {
            params.push(("loanId", s));
        }
        if let Some(s) = order_currency {
            params.push(("orderCurrency", s));
        }
        if let Some(s) = term {
            params.push(("term", s));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        if let Some(ref s) = cursor_str {
            params.push(("cursor", s.as_str()));
        }
        self.get("/v5/crypto-loan-fixed/borrow-contract-info", &params)
            .await
    }

    /// Get Borrow Order Info
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn get_fixed_borrow_order_info(
        &self,
        order_id: Option<&str>,
        order_currency: Option<&str>,
        state: Option<&str>,
        term: Option<&str>,
        limit: Option<i64>,
        cursor: Option<i64>,
    ) -> Result<serde_json::Value> {
        let limit_str = limit.map(|v| v.to_string());
        let cursor_str = cursor.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = order_id {
            params.push(("orderId", s));
        }
        if let Some(s) = order_currency {
            params.push(("orderCurrency", s));
        }
        if let Some(s) = state {
            params.push(("state", s));
        }
        if let Some(s) = term {
            params.push(("term", s));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        if let Some(ref s) = cursor_str {
            params.push(("cursor", s.as_str()));
        }
        self.get("/v5/crypto-loan-fixed/borrow-order-info", &params)
            .await
    }

    /// Get Borrow Market Quotes
    pub async fn get_fixed_borrow_order_quote(
        &self,
        order_currency: Option<&str>,
        term: Option<&str>,
        order_by: Option<&str>,
        sort: Option<i64>,
        limit: Option<i64>,
    ) -> Result<GetFixedBorrowOrderQuoteResponse> {
        let sort_str = sort.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = order_currency {
            params.push(("orderCurrency", s));
        }
        if let Some(s) = term {
            params.push(("term", s));
        }
        if let Some(s) = order_by {
            params.push(("orderBy", s));
        }
        if let Some(ref s) = sort_str {
            params.push(("sort", s.as_str()));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        self.get_public("/v5/crypto-loan-fixed/borrow-order-quote", &params)
            .await
    }

    /// Get Renewal Information
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn get_fixed_renew_info(
        &self,
        order_id: Option<&str>,
        order_currency: Option<&str>,
        limit: Option<i64>,
        cursor: Option<i64>,
    ) -> Result<serde_json::Value> {
        let limit_str = limit.map(|v| v.to_string());
        let cursor_str = cursor.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = order_id {
            params.push(("orderId", s));
        }
        if let Some(s) = order_currency {
            params.push(("orderCurrency", s));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        if let Some(ref s) = cursor_str {
            params.push(("cursor", s.as_str()));
        }
        self.get("/v5/crypto-loan-fixed/renew-info", &params).await
    }

    /// Get Crypto Loan Position
    pub async fn get_loan_position(&self) -> Result<GetLoanPositionResponse> {
        self.get("/v5/crypto-loan-common/position", &[]).await
    }

    /// Get Loanable Currency Data
    pub async fn get_loanable_data(
        &self,
        currency: Option<&str>,
        vip_level: Option<&str>,
    ) -> Result<GetLoanableDataResponse> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = currency {
            params.push(("currency", s));
        }
        if let Some(s) = vip_level {
            params.push(("vipLevel", s));
        }
        // Public endpoint per Bybit V5 docs ("Does not need authentication"):
        // https://bybit-exchange.github.io/docs/v5/new-crypto-loan/loan-coin
        self.get_public("/v5/crypto-loan-common/loanable-data", &params)
            .await
    }

    /// Get Max Collateral Redeem Amount
    pub async fn get_max_collateral_amount(
        &self,
        currency: &str,
    ) -> Result<GetMaxCollateralAmountResponse> {
        let params = vec![("currency", currency)];
        self.get("/v5/crypto-loan-common/max-collateral-amount", &params)
            .await
    }

    /// Repay with Collateral (Fixed)
    pub async fn repay_fixed_collateral(
        &self,
        params: PostCryptoLoanFixedRepayCollateralParams,
    ) -> Result<PostCryptoLoanFixedRepayCollateralResponse> {
        self.post("/v5/crypto-loan-fixed/repay-collateral", &params)
            .await
    }

    /// Create Supply Order
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn create_fixed_supply(
        &self,
        params: PostCryptoLoanFixedSupplyParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/crypto-loan-fixed/supply", &params).await
    }

    /// Cancel Supply Order
    pub async fn cancel_fixed_supply_order(
        &self,
        params: PostCryptoLoanFixedSupplyOrderCancelParams,
    ) -> Result<PostCryptoLoanFixedSupplyOrderCancelResponse> {
        self.post("/v5/crypto-loan-fixed/supply-order-cancel", &params)
            .await
    }

    /// Create Flexible Borrow Order
    pub async fn create_flexible_borrow(
        &self,
        params: PostCryptoLoanFlexibleBorrowParams,
    ) -> Result<PostCryptoLoanFlexibleBorrowResponse> {
        self.post("/v5/crypto-loan-flexible/borrow", &params).await
    }

    /// Repay Flexible Loan
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn repay_flexible_loan(
        &self,
        params: PostCryptoLoanFlexibleRepayParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/crypto-loan-flexible/repay", &params).await
    }

    /// Repay with Collateral (Flexible)
    pub async fn repay_flexible_collateral(
        &self,
        params: PostCryptoLoanFlexibleRepayCollateralParams,
    ) -> Result<PostCryptoLoanFlexibleRepayCollateralResponse> {
        self.post("/v5/crypto-loan-flexible/repay-collateral", &params)
            .await
    }

    /// Renew Loan
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn renew_fixed_loan(
        &self,
        params: RenewFixedLoanParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/crypto-loan-fixed/renew", &params).await
    }
}

impl BybitClient {
    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    pub async fn get_crypto_loan_fixed_available_inventory(
        &self,
        currency: &str,
        term: &str,
        annual_rate: &str,
    ) -> Result<serde_json::Value> {
        let params = vec![
            ("currency", currency),
            ("term", term),
            ("annualRate", annual_rate),
        ];
        self.get("/v5/crypto-loan-fixed/available-inventory", &params)
            .await
    }

    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    pub async fn get_crypto_loan_flexible_available_inventory(
        &self,
        currency: &str,
    ) -> Result<serde_json::Value> {
        let params = vec![("currency", currency)];
        self.get("/v5/crypto-loan-flexible/available-inventory", &params)
            .await
    }
}
