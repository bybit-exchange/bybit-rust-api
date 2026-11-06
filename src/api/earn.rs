//! Earn API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::earn::*;
use crate::models::Category;

impl BybitClient {
    pub async fn add_liquidity(&self, params: AddLiquidityParams) -> Result<AddLiquidityResponse> {
        self.post("/v5/earn/liquidity-mining/add-liquidity", &params)
            .await
    }

    // FIXME(gen-sdk-rust): plan agent referenced AddMarginParams without
    // actually emitting the type to models/earn.rs. Falling back to
    // serde_json::Value so the SDK compiles; the caller constructs the body
    // manually. Re-run gen-sdk-rust after fixing plan output to recover the
    // typed signature.
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn add_liquidity_mining_margin(
        &self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.post("/v5/earn/liquidity-mining/add-margin", &params)
            .await
    }

    pub async fn claim_liquidity_interest(
        &self,
        params: ClaimLiquidityInterestParams,
    ) -> Result<ClaimLiquidityInterestResponse> {
        self.post("/v5/earn/liquidity-mining/claim-interest", &params)
            .await
    }

    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn get_advance_earn_order(
        &self,
        category: Category,
        product_id: Option<i64>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetAdvanceEarnOrderResponse> {
        let product_id_str = product_id.map(|v| v.to_string());
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(ref v) = product_id_str {
            params.push(("productId", v.as_str()));
        }
        if let Some(v) = order_id {
            params.push(("orderId", v));
        }
        if let Some(v) = order_link_id {
            params.push(("orderLinkId", v));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/advance/order", &params).await
    }

    pub async fn get_advance_earn_position(
        &self,
        category: Category,
        product_id: Option<i64>,
        coin: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetAdvanceEarnPositionResponse> {
        let product_id_str = product_id.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(ref v) = product_id_str {
            params.push(("productId", v.as_str()));
        }
        if let Some(v) = coin {
            params.push(("coin", v));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/advance/position", &params).await
    }

    pub async fn get_advance_earn_product(
        &self,
        category: Category,
        coin: Option<&str>,
        duration: Option<&str>,
    ) -> Result<GetAdvanceEarnProductResponse> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(v) = coin {
            params.push(("coin", v));
        }
        if let Some(v) = duration {
            params.push(("duration", v));
        }

        self.get_public("/v5/earn/advance/product", &params).await
    }

    pub async fn get_advance_earn_product_extra_info(
        &self,
        category: Category,
        product_id: Option<i64>,
    ) -> Result<GetAdvanceEarnProductExtraInfoResponse> {
        let product_id_str = product_id.map(|v| v.to_string());

        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(ref v) = product_id_str {
            params.push(("productId", v.as_str()));
        }

        self.get_public("/v5/earn/advance/product-extra-info", &params)
            .await
    }

    pub async fn get_double_win_leverage(
        &self,
        product_id: i64,
        initial_price: &str,
        lower_price: &str,
        upper_price: &str,
    ) -> Result<GetDoubleWinLeverageResponse> {
        let product_id_str = product_id.to_string();
        let params = vec![
            ("productId", product_id_str.as_str()),
            ("initialPrice", initial_price),
            ("lowerPrice", lower_price),
            ("upperPrice", upper_price),
        ];

        self.get("/v5/earn/advance/double-win-leverage", &params)
            .await
    }

    pub async fn get_earn_apr_history(
        &self,
        category: Category,
        product_id: &str,
        start_time: i64,
        end_time: i64,
    ) -> Result<GetEarnAprHistoryResponse> {
        let cat_str = category.to_string();
        let start_time_str = start_time.to_string();
        let end_time_str = end_time.to_string();
        let params = vec![
            ("category", cat_str.as_str()),
            ("productId", product_id),
            ("startTime", start_time_str.as_str()),
            ("endTime", end_time_str.as_str()),
        ];

        self.get_public("/v5/earn/apr-history", &params).await
    }

    pub async fn get_earn_hourly_yield_history(
        &self,
        category: Category,
        product_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetEarnHourlyYieldHistoryResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(v) = product_id {
            params.push(("productId", v));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/hourly-yield", &params).await
    }

    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn get_earn_order_history(
        &self,
        category: Category,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        product_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetEarnOrderHistoryResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(v) = order_id {
            params.push(("orderId", v));
        }
        if let Some(v) = order_link_id {
            params.push(("orderLinkId", v));
        }
        if let Some(v) = product_id {
            params.push(("productId", v));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/order", &params).await
    }

    pub async fn get_earn_position(
        &self,
        category: Category,
        product_id: Option<&str>,
        coin: Option<&str>,
    ) -> Result<GetEarnPositionResponse> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(v) = product_id {
            params.push(("productId", v));
        }
        if let Some(v) = coin {
            params.push(("coin", v));
        }

        self.get("/v5/earn/position", &params).await
    }

    pub async fn get_earn_product(
        &self,
        category: Category,
        coin: Option<&str>,
    ) -> Result<GetEarnProductResponse> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(v) = coin {
            params.push(("coin", v));
        }

        self.get_public("/v5/earn/product", &params).await
    }

    pub async fn get_earn_yield_history(
        &self,
        category: Category,
        product_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetEarnYieldHistoryResponse> {
        let product_id_str = product_id.map(|v| v.to_string());
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(ref v) = product_id_str {
            params.push(("productId", v.as_str()));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/yield", &params).await
    }

    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn get_fixed_term_order(
        &self,
        order_type: Option<&str>,
        product_id: Option<&str>,
        category: Option<Category>,
        order_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetFixedTermOrderResponse> {
        let cat_str = category.map(|c| c.to_string());
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = order_type {
            params.push(("orderType", v));
        }
        if let Some(v) = product_id {
            params.push(("productId", v));
        }
        if let Some(ref v) = cat_str {
            params.push(("category", v.as_str()));
        }
        if let Some(v) = order_id {
            params.push(("orderId", v));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/fixed-term/order", &params).await
    }

    pub async fn get_fixed_term_position(
        &self,
        product_id: Option<&str>,
        category: Option<Category>,
        coin: Option<&str>,
    ) -> Result<GetFixedTermPositionResponse> {
        let cat_str = category.map(|c| c.to_string());
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = product_id {
            params.push(("productId", v));
        }
        if let Some(ref v) = cat_str {
            params.push(("category", v.as_str()));
        }
        if let Some(v) = coin {
            params.push(("coin", v));
        }

        self.get("/v5/earn/fixed-term/position", &params).await
    }

    pub async fn get_fixed_term_product(
        &self,
        coin: Option<&str>,
    ) -> Result<GetFixedTermProductResponse> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = coin {
            params.push(("coin", v));
        }

        self.get_public("/v5/earn/fixed-term/product", &params)
            .await
    }

    pub async fn get_hold_to_earn_product(&self) -> Result<GetHoldToEarnProductResponse> {
        self.get_public("/v5/earn/hold-to-earn/product", &[]).await
    }

    pub async fn get_hold_to_earn_yield_history(
        &self,
        time_start: Option<i64>,
        time_end: Option<i64>,
        limit: i64,
        cursor: Option<&str>,
    ) -> Result<GetHoldToEarnYieldHistoryResponse> {
        let time_start_str = time_start.map(|v| v.to_string());
        let time_end_str = time_end.map(|v| v.to_string());
        let limit_str = limit.to_string();

        let mut params = vec![("limit", limit_str.as_str())];
        if let Some(ref v) = time_start_str {
            params.push(("timeStart", v.as_str()));
        }
        if let Some(ref v) = time_end_str {
            params.push(("timeEnd", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/hold-to-earn/yield-history", &params)
            .await
    }

    pub async fn get_liquidity_mining_liquidation_records(
        &self,
        base_coin: Option<&str>,
        quote_coin: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetLiquidityMiningLiquidationRecordsResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = base_coin {
            params.push(("baseCoin", v));
        }
        if let Some(v) = quote_coin {
            params.push(("quoteCoin", v));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/liquidity-mining/liquidation-records", &params)
            .await
    }

    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn get_liquidity_mining_orders(
        &self,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        product_id: Option<&str>,
        order_type: Option<&str>,
        status: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetLiquidityMiningOrdersResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = order_id {
            params.push(("orderId", v));
        }
        if let Some(v) = order_link_id {
            params.push(("orderLinkId", v));
        }
        if let Some(v) = product_id {
            params.push(("productId", v));
        }
        if let Some(v) = order_type {
            params.push(("orderType", v));
        }
        if let Some(v) = status {
            params.push(("status", v));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/liquidity-mining/order", &params).await
    }

    pub async fn get_liquidity_mining_positions(
        &self,
        product_id: Option<&str>,
        base_coin: Option<&str>,
    ) -> Result<GetLiquidityMiningPositionsResponse> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = product_id {
            params.push(("productId", v));
        }
        if let Some(v) = base_coin {
            params.push(("baseCoin", v));
        }

        self.get("/v5/earn/liquidity-mining/position", &params)
            .await
    }

    pub async fn get_liquidity_mining_products(
        &self,
        base_coin: Option<&str>,
        quote_coin: Option<&str>,
    ) -> Result<GetLiquidityMiningProductsResponse> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = base_coin {
            params.push(("baseCoin", v));
        }
        if let Some(v) = quote_coin {
            params.push(("quoteCoin", v));
        }

        self.get_public("/v5/earn/liquidity-mining/product", &params)
            .await
    }

    pub async fn get_liquidity_mining_yield_records(
        &self,
        base_coin: Option<&str>,
        quote_coin: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetLiquidityMiningYieldRecordsResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = base_coin {
            params.push(("baseCoin", v));
        }
        if let Some(v) = quote_coin {
            params.push(("quoteCoin", v));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/liquidity-mining/yield-records", &params)
            .await
    }

    pub async fn get_rwa_nav_chart(
        &self,
        product_id: i64,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<GetRwaNavChartResponse> {
        let product_id_str = product_id.to_string();
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());

        let mut params = vec![("productId", product_id_str.as_str())];
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }

        self.get_public("/v5/earn/rwa/nav-chart", &params).await
    }

    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn get_rwa_order_list(
        &self,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        order_type: Option<&str>,
        product_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetRwaOrderListResponse> {
        let product_id_str = product_id.map(|v| v.to_string());
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = order_id {
            params.push(("orderId", v));
        }
        if let Some(v) = order_link_id {
            params.push(("orderLinkId", v));
        }
        if let Some(v) = order_type {
            params.push(("orderType", v));
        }
        if let Some(ref v) = product_id_str {
            params.push(("productId", v.as_str()));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/rwa/order", &params).await
    }

    pub async fn get_rwa_position_list(&self) -> Result<GetRwaPositionListResponse> {
        self.get("/v5/earn/rwa/position", &[]).await
    }

    pub async fn get_rwa_product_list(
        &self,
        coin: Option<&str>,
    ) -> Result<GetRwaProductListResponse> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = coin {
            params.push(("coin", v));
        }

        self.get_public("/v5/earn/rwa/product", &params).await
    }

    pub async fn get_smart_leverage_redeem_est_amount_list(
        &self,
        category: Category,
        position_ids: &str,
    ) -> Result<GetSmartLeverageRedeemEstAmountListResponse> {
        let cat_str = category.to_string();
        let params = vec![
            ("category", cat_str.as_str()),
            ("positionIds", position_ids),
        ];

        self.get("/v5/earn/advance/get-redeem-est-amount-list", &params)
            .await
    }

    pub async fn get_token_daily_yield(
        &self,
        coin: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<GetTokenDailyYieldResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params = vec![("coin", coin)];
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }

        self.get("/v5/earn/token/yield", &params).await
    }

    pub async fn get_token_historical_apr(
        &self,
        coin: &str,
        range: i64,
    ) -> Result<GetTokenHistoricalAprResponse> {
        let range_str = range.to_string();
        let params = vec![("coin", coin), ("range", range_str.as_str())];

        self.get_public("/v5/earn/token/history-apr", &params).await
    }

    pub async fn get_token_hourly_yield(
        &self,
        coin: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<GetTokenHourlyYieldResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params = vec![("coin", coin)];
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }

        self.get("/v5/earn/token/hourly-yield", &params).await
    }

    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn get_token_order_list(
        &self,
        coin: &str,
        order_link_id: Option<&str>,
        order_id: Option<&str>,
        order_type: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        cursor: Option<&str>,
        limit: Option<i64>,
    ) -> Result<GetTokenOrderListResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params = vec![("coin", coin)];
        if let Some(v) = order_link_id {
            params.push(("orderLinkId", v));
        }
        if let Some(v) = order_id {
            params.push(("orderId", v));
        }
        if let Some(v) = order_type {
            params.push(("orderType", v));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }

        self.get("/v5/earn/token/order", &params).await
    }

    pub async fn get_token_position(&self, coin: &str) -> Result<GetTokenPositionResponse> {
        let params = vec![("coin", coin)];
        self.get("/v5/earn/token/position", &params).await
    }

    pub async fn get_token_product(&self, coin: &str) -> Result<GetTokenProductResponse> {
        let params = vec![("coin", coin)];
        self.get_public("/v5/earn/token/product", &params).await
    }

    pub async fn list_earn_coupons(&self, category: Category) -> Result<ListEarnCouponsResponse> {
        let cat_str = category.to_string();
        let params = vec![("category", cat_str.as_str())];
        self.get("/v5/earn/coupons", &params).await
    }

    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn modify_earn_position(
        &self,
        params: ModifyEarnPositionParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/earn/position/modify", &params).await
    }

    pub async fn place_advance_earn_order(
        &self,
        params: PlaceAdvanceEarnOrderParams,
    ) -> Result<PlaceAdvanceEarnOrderResponse> {
        self.post("/v5/earn/advance/place-order", &params).await
    }

    pub async fn place_earn_order(
        &self,
        params: PlaceEarnOrderParams,
    ) -> Result<PlaceEarnOrderResponse> {
        self.post("/v5/earn/place-order", &params).await
    }

    pub async fn place_fixed_term_order(
        &self,
        params: PlaceFixedTermOrderParams,
    ) -> Result<PlaceFixedTermOrderResponse> {
        self.post("/v5/earn/fixed-term/place-order", &params).await
    }

    pub async fn place_rwa_order(
        &self,
        params: PlaceRwaOrderParams,
    ) -> Result<PlaceRwaOrderResponse> {
        self.post("/v5/earn/rwa/place-order", &params).await
    }

    pub async fn place_token_order(
        &self,
        params: PlaceTokenOrderParams,
    ) -> Result<PlaceTokenOrderResponse> {
        self.post("/v5/earn/token/place-order", &params).await
    }

    pub async fn pwm_asset_trend(
        &self,
        plan_id: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<PwmAssetTrendResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());

        let mut params = vec![("planId", plan_id)];
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }

        self.get("/v5/earn/pwm/investment-plan/asset-trend", &params)
            .await
    }

    pub async fn pwm_claim(&self, params: PwmClaimParams) -> Result<PwmClaimResponse> {
        self.post("/v5/earn/pwm/investment-plan/claim", &params)
            .await
    }

    pub async fn pwm_create_custom_plan(
        &self,
        params: PwmCreateCustomPlanParams,
    ) -> Result<PwmCreateCustomPlanResponse> {
        self.post("/v5/earn/pwm/customize-plan/create", &params)
            .await
    }

    pub async fn pwm_fund_nav(
        &self,
        fund_id: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<PwmFundNavResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());

        let mut params = vec![("fundId", fund_id)];
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }

        self.get("/v5/earn/pwm/investment-plan/fund-nav", &params)
            .await
    }

    pub async fn pwm_fund_transfer(
        &self,
        params: PwmFundTransferParams,
    ) -> Result<PwmFundTransferResponse> {
        self.post("/v5/earn/pwm/fund-transfer", &params).await
    }

    pub async fn pwm_get_new_plan_detail(
        &self,
        plan_id: &str,
    ) -> Result<PwmGetNewPlanDetailResponse> {
        let params = vec![("planId", plan_id)];
        self.get("/v5/earn/pwm/investment-plan/new-plan", &params)
            .await
    }

    pub async fn pwm_get_plan_detail(&self, plan_id: &str) -> Result<PwmGetPlanDetailResponse> {
        let params = vec![("planId", plan_id)];
        self.get("/v5/earn/pwm/investment-plan/detail", &params)
            .await
    }

    pub async fn pwm_inst_create_fund(
        &self,
        params: PwmInstCreateFundParams,
    ) -> Result<PwmInstCreateFundResponse> {
        self.post("/v5/earn/pwm/asset-manager/create-fund", &params)
            .await
    }

    pub async fn pwm_inst_create_investment_plan(
        &self,
        params: PwmInstCreateInvestmentPlanParams,
    ) -> Result<PwmInstCreateInvestmentPlanResponse> {
        self.post("/v5/earn/pwm/asset-manager/create-investment-plan", &params)
            .await
    }

    pub async fn pwm_inst_create_sub_account(
        &self,
        params: PwmInstCreateSubAccountParams,
    ) -> Result<PwmInstCreateSubAccountResponse> {
        self.post("/v5/earn/pwm/asset-manager/create-sub-account", &params)
            .await
    }

    pub async fn pwm_inst_get_investment_plans(
        &self,
        plan_id: Option<&str>,
        status: Option<&str>,
        subscription_uid: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<PwmInstGetInvestmentPlansResponse> {
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = plan_id {
            params.push(("planId", v));
        }
        if let Some(v) = status {
            params.push(("status", v));
        }
        if let Some(v) = subscription_uid {
            params.push(("subscriptionUid", v));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/pwm/asset-manager/get-investment-plan", &params)
            .await
    }

    pub async fn pwm_inst_list_funds(
        &self,
        fund_id: Option<&str>,
        coin: Option<&str>,
        status: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<PwmInstListFundsResponse> {
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = fund_id {
            params.push(("fundId", v));
        }
        if let Some(v) = coin {
            params.push(("coin", v));
        }
        if let Some(v) = status {
            params.push(("status", v));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/pwm/asset-manager/all-funds", &params)
            .await
    }

    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn pwm_inst_list_orders(
        &self,
        fund_id: Option<&str>,
        order_type: Option<&str>,
        status: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<PwmInstListOrdersResponse> {
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = fund_id {
            params.push(("fundId", v));
        }
        if let Some(v) = order_type {
            params.push(("orderType", v));
        }
        if let Some(v) = status {
            params.push(("status", v));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/pwm/asset-manager/all-order", &params)
            .await
    }

    pub async fn pwm_inst_manage_investment_plan(
        &self,
        params: PwmInstManageInvestmentPlanParams,
    ) -> Result<PwmInstManageInvestmentPlanResponse> {
        self.post("/v5/earn/pwm/asset-manager/manage-investment-plan", &params)
            .await
    }

    pub async fn pwm_inst_manage_order(
        &self,
        params: PwmInstManageOrderParams,
    ) -> Result<PwmInstManageOrderResponse> {
        self.post("/v5/earn/pwm/asset-manager/manage-order", &params)
            .await
    }

    pub async fn pwm_inst_settle_profit(
        &self,
        params: PwmInstSettleProfitParams,
    ) -> Result<PwmInstSettleProfitResponse> {
        self.post("/v5/earn/pwm/asset-manager/settle-profit", &params)
            .await
    }

    pub async fn pwm_invest_more(
        &self,
        params: PwmInvestMoreParams,
    ) -> Result<PwmInvestMoreResponse> {
        self.post("/v5/earn/pwm/investment-plan/invest-more", &params)
            .await
    }

    pub async fn pwm_list_investment_plans(
        &self,
        plan_id: Option<&str>,
        status: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<PwmListInvestmentPlansResponse> {
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = plan_id {
            params.push(("planId", v));
        }
        if let Some(v) = status {
            params.push(("status", v));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }

        self.get("/v5/earn/pwm/investment-plan/all", &params).await
    }

    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn pwm_list_order(
        &self,
        plan_id: Option<&str>,
        category: Option<Category>,
        order_type: Option<&str>,
        status: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
        order_link_id: Option<&str>,
    ) -> Result<PwmListOrderResponse> {
        let cat_str = category.map(|c| c.to_string());
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = plan_id {
            params.push(("planId", v));
        }
        if let Some(ref v) = cat_str {
            params.push(("category", v.as_str()));
        }
        if let Some(v) = order_type {
            params.push(("type", v));
        }
        if let Some(v) = status {
            params.push(("status", v));
        }
        if let Some(ref v) = start_time_str {
            params.push(("startTime", v.as_str()));
        }
        if let Some(ref v) = end_time_str {
            params.push(("endTime", v.as_str()));
        }
        if let Some(ref v) = limit_str {
            params.push(("limit", v.as_str()));
        }
        if let Some(v) = cursor {
            params.push(("cursor", v));
        }
        if let Some(v) = order_link_id {
            params.push(("orderLinkId", v));
        }

        self.get("/v5/earn/pwm/investment-plan/order", &params)
            .await
    }

    pub async fn pwm_list_product_cards(&self) -> Result<PwmListProductCardsResponse> {
        // Public endpoint per Bybit V5 docs:
        // https://bybit-exchange.github.io/docs/v5/finance/pwm/customize-plan/product
        self.get_public("/v5/earn/pwm/customize-plan/product", &[])
            .await
    }

    pub async fn pwm_query_fund_transfer_result(
        &self,
        transfer_id: Option<&str>,
        from_user_id: Option<i64>,
    ) -> Result<PwmQueryFundTransferResultResponse> {
        let from_user_id_str = from_user_id.map(|v| v.to_string());

        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(v) = transfer_id {
            params.push(("transferId", v));
        }
        if let Some(ref v) = from_user_id_str {
            params.push(("fromUserId", v.as_str()));
        }

        self.get("/v5/earn/pwm/query-fund-transfer-result", &params)
            .await
    }

    pub async fn pwm_redeem(&self, params: PwmRedeemParams) -> Result<PwmRedeemResponse> {
        self.post("/v5/earn/pwm/investment-plan/redeem", &params)
            .await
    }

    pub async fn pwm_subscribe(&self, params: PwmSubscribeParams) -> Result<PwmSubscribeResponse> {
        self.post("/v5/earn/pwm/investment-plan/subscribe", &params)
            .await
    }

    pub async fn redeem_fixed_term(
        &self,
        params: RedeemFixedTermParams,
    ) -> Result<RedeemFixedTermResponse> {
        self.post("/v5/earn/fixed-term/redeem", &params).await
    }

    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn reinvest_liquidity(
        &self,
        params: ReinvestLiquidityParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/earn/liquidity-mining/reinvest", &params)
            .await
    }

    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn remove_liquidity(
        &self,
        params: RemoveLiquidityParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/earn/liquidity-mining/remove-liquidity", &params)
            .await
    }

    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn set_fixed_term_auto_invest(
        &self,
        params: SetFixedTermAutoInvestParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/earn/fixed-term/position/auto-invest", &params)
            .await
    }
}

fn auto_savings_params<'a>(coins: Option<&'a [&'a str]>) -> Vec<(&'static str, &'a str)> {
    coins
        .unwrap_or_default()
        .iter()
        .map(|coin| ("coins", *coin))
        .collect()
}

impl BybitClient {
    pub async fn list_earn_auto_savings(
        &self,
        coins: Option<&[&str]>,
    ) -> Result<ListEarnAutoSavingsResponse> {
        let params = auto_savings_params(coins);
        self.get("/v5/earn/flexible-saving/auto-savings", &params)
            .await
    }

    pub async fn edit_earn_auto_savings(
        &self,
        params: EditEarnAutoSavingsParams,
    ) -> Result<EditEarnAutoSavingsResponse> {
        self.post("/v5/earn/flexible-saving/auto-savings", &params)
            .await
    }
}

#[cfg(test)]
mod auto_savings_tests {
    use super::auto_savings_params;

    #[test]
    fn auto_savings_uses_repeated_coin_parameters() {
        assert_eq!(
            auto_savings_params(Some(&["BTC", "ETH"])),
            vec![("coins", "BTC"), ("coins", "ETH")]
        );
        assert!(auto_savings_params(None).is_empty());
    }
}
