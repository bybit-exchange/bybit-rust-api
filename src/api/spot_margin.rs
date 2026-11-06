//! Spot Margin API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::spot_margin::*;

impl BybitClient {
    /// Get Historical Interest Rate.
    pub async fn get_historical_interest_rate(
        &self,
        currency: &str,
        vip_level: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<GetHistoricalInterestRateResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let mut params = vec![("currency", currency)];
        if let Some(v) = vip_level {
            params.push(("vipLevel", v));
        }
        if let Some(s) = start_time_str.as_deref() {
            params.push(("startTime", s));
        }
        if let Some(s) = end_time_str.as_deref() {
            params.push(("endTime", s));
        }
        self.get("/v5/spot-margin-trade/interest-rate-history", &params)
            .await
    }

    /// Get Position Tiers.
    pub async fn get_position_tiers(
        &self,
        currency: Option<&str>,
    ) -> Result<GetPositionTiersResponse> {
        let mut params = vec![];
        if let Some(c) = currency {
            params.push(("currency", c));
        }
        self.get("/v5/spot-margin-trade/position-tiers", &params)
            .await
    }

    /// Get Tiered Collateral Ratio.
    pub async fn get_tiered_collateral_ratio(
        &self,
        currency: Option<&str>,
    ) -> Result<GetTieredCollateralRatioResponse> {
        let mut params = vec![];
        if let Some(c) = currency {
            params.push(("currency", c));
        }
        self.get_public("/v5/spot-margin-trade/collateral", &params)
            .await
    }

    /// Get VIP Margin Data.
    pub async fn get_vip_margin_data(
        &self,
        vip_level: Option<&str>,
        currency: Option<&str>,
    ) -> Result<GetVipMarginDataResponse> {
        let mut params = vec![];
        if let Some(v) = vip_level {
            params.push(("vipLevel", v));
        }
        if let Some(c) = currency {
            params.push(("currency", c));
        }
        self.get_public("/v5/spot-margin-trade/data", &params).await
    }
}

impl BybitClient {
    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    pub async fn spot_margin_set_leverage(
        &self,
        params: SpotMarginSetLeverageParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/spot-margin-trade/set-leverage", &params)
            .await
    }

    pub async fn spot_margin_switch_mode(
        &self,
        params: SpotMarginSwitchModeParams,
    ) -> Result<SpotMarginSwitchModeResponse> {
        self.post("/v5/spot-margin-trade/switch-mode", &params)
            .await
    }

    pub async fn get_spot_margin_trade_coin_state(
        &self,
        currency: Option<&str>,
    ) -> Result<GetSpotMarginTradeCoinStateResponse> {
        let mut params = vec![];
        if let Some(value) = currency {
            params.push(("currency", value));
        }
        self.get("/v5/spot-margin-trade/coinstate", &params).await
    }

    pub async fn get_spot_margin_trade_flexible_available_inventory(
        &self,
        currency: &str,
    ) -> Result<GetSpotMarginTradeFlexibleAvailableInventoryResponse> {
        let params = vec![("currency", currency)];
        self.get(
            "/v5/spot-margin-trade/flexible-available-inventory",
            &params,
        )
        .await
    }

    pub async fn get_spot_margin_trade_auto_repay_mode(
        &self,
        currency: Option<&str>,
    ) -> Result<GetSpotMarginTradeAutoRepayModeResponse> {
        let mut params = vec![];
        if let Some(value) = currency {
            params.push(("currency", value));
        }
        self.get("/v5/spot-margin-trade/get-auto-repay-mode", &params)
            .await
    }

    pub async fn get_spot_margin_trade_max_borrowable(
        &self,
        currency: &str,
    ) -> Result<GetSpotMarginTradeMaxBorrowableResponse> {
        let params = vec![("currency", currency)];
        self.get("/v5/spot-margin-trade/max-borrowable", &params)
            .await
    }

    pub async fn get_spot_margin_trade_repayment_available_amount(
        &self,
        currency: &str,
    ) -> Result<GetSpotMarginTradeRepaymentAvailableAmountResponse> {
        let params = vec![("currency", currency)];
        self.get("/v5/spot-margin-trade/repayment-available-amount", &params)
            .await
    }

    pub async fn get_spot_margin_trade_state(&self) -> Result<GetSpotMarginTradeStateResponse> {
        let params = vec![];
        self.get("/v5/spot-margin-trade/state", &params).await
    }

    pub async fn query_fixed_available_inventory(
        &self,
        currency: &str,
        term: &str,
        annual_rate: &str,
    ) -> Result<QueryFixedAvailableInventoryResponse> {
        let params = vec![
            ("currency", currency),
            ("term", term),
            ("annualRate", annual_rate),
        ];
        self.get("/v5/spot-margin-trade/fixed-available-inventory", &params)
            .await
    }

    pub async fn query_fixed_borrow_contracts(
        &self,
        order_id: Option<&str>,
        order_currency: Option<&str>,
        term: Option<&str>,
        limit: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<QueryFixedBorrowContractsResponse> {
        let mut params = vec![];
        if let Some(value) = order_id {
            params.push(("orderId", value));
        }
        if let Some(value) = order_currency {
            params.push(("orderCurrency", value));
        }
        if let Some(value) = term {
            params.push(("term", value));
        }
        if let Some(value) = limit {
            params.push(("limit", value));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/spot-margin-trade/fixedborrow-contract-info", &params)
            .await
    }

    pub async fn query_fixed_borrow_orders(
        &self,
        order_id: Option<&str>,
        order_currency: Option<&str>,
        state: Option<&str>,
        term: Option<&str>,
        limit: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<QueryFixedBorrowOrdersResponse> {
        let mut params = vec![];
        if let Some(value) = order_id {
            params.push(("orderId", value));
        }
        if let Some(value) = order_currency {
            params.push(("orderCurrency", value));
        }
        if let Some(value) = state {
            params.push(("state", value));
        }
        if let Some(value) = term {
            params.push(("term", value));
        }
        if let Some(value) = limit {
            params.push(("limit", value));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/spot-margin-trade/fixedborrow-order-info", &params)
            .await
    }

    pub async fn query_fixed_borrow_market(
        &self,
        order_currency: &str,
        term: Option<&str>,
        order_by: &str,
        sort: Option<i64>,
        limit: Option<i64>,
    ) -> Result<QueryFixedBorrowMarketResponse> {
        let mut params = vec![("orderCurrency", order_currency), ("orderBy", order_by)];
        if let Some(value) = term {
            params.push(("term", value));
        }
        let sort_string;
        if let Some(value) = sort {
            sort_string = value.to_string();
            params.push(("sort", sort_string.as_str()));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        self.get("/v5/spot-margin-trade/fixedborrow-order-quote", &params)
            .await
    }

    pub async fn renew_fixed_borrow(
        &self,
        params: RenewFixedBorrowParams,
    ) -> Result<RenewFixedBorrowResponse> {
        self.post("/v5/spot-margin-trade/fixedborrow-renew", &params)
            .await
    }

    pub async fn account_fixed_borrow(
        &self,
        params: AccountFixedBorrowParams,
    ) -> Result<AccountFixedBorrowResponse> {
        self.post("/v5/spot-margin-trade/fixedborrow", &params)
            .await
    }

    pub async fn query_borrow_liability(
        &self,
        currency: &str,
    ) -> Result<QueryBorrowLiabilityResponse> {
        let params = vec![("currency", currency)];
        self.get("/v5/spot-margin-trade/liability", &params).await
    }

    pub async fn set_auto_repay_mode(
        &self,
        params: SetAutoRepayModeParams,
    ) -> Result<SetAutoRepayModeResponse> {
        self.post("/v5/spot-margin-trade/set-auto-repay-mode", &params)
            .await
    }
}

impl BybitClient {
    pub async fn query_fixed_borrow_penalty_interest(
        &self,
        order_id: Option<&str>,
        order_currency: Option<&str>,
        limit: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<QueryFixedBorrowPenaltyInterestResponse> {
        let mut params = vec![];
        if let Some(value) = order_id {
            params.push(("orderId", value));
        }
        if let Some(value) = order_currency {
            params.push(("orderCurrency", value));
        }
        if let Some(value) = limit {
            params.push(("limit", value));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get(
            "/v5/spot-margin-trade/fixedborrow-interest-accrued",
            &params,
        )
        .await
    }

    pub async fn cancel_fixed_borrow_order_post(
        &self,
        params: CancelFixedBorrowOrderPostParams,
    ) -> Result<CancelFixedBorrowOrderPostResponse> {
        self.post("/v5/spot-margin-trade/fixedborrow-order-cancel", &params)
            .await
    }
}
