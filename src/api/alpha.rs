use crate::client::BybitClient;
use crate::error::Result;
use crate::models::alpha::*;

impl BybitClient {
    pub async fn get_lporder_list(
        &self,
        params: GetLporderListParams,
    ) -> Result<GetLporderListResponse> {
        self.post("/v5/alpha/lp/order-list", &params).await
    }

    pub async fn get_lppay_token_list(
        &self,
        params: GetLppayTokenListParams,
    ) -> Result<GetLppayTokenListResponse> {
        self.post("/v5/alpha/lp/pay-token-list", &params).await
    }

    pub async fn get_lppay_token_price(
        &self,
        params: GetLppayTokenPriceParams,
    ) -> Result<GetLppayTokenPriceResponse> {
        self.post("/v5/alpha/lp/pay-token-price", &params).await
    }

    pub async fn get_lppool_info(
        &self,
        params: GetLppoolInfoParams,
    ) -> Result<GetLppoolInfoResponse> {
        self.post("/v5/alpha/lp/pool-info", &params).await
    }

    pub async fn get_lppool_list(
        &self,
        params: GetLppoolListParams,
    ) -> Result<GetLppoolListResponse> {
        self.post("/v5/alpha/lp/pool-list", &params).await
    }

    pub async fn get_lpposition_list(
        &self,
        params: GetLppositionListParams,
    ) -> Result<GetLppositionListResponse> {
        self.post("/v5/alpha/lp/position-list", &params).await
    }

    pub async fn execute_lpredeem(
        &self,
        params: ExecuteLpredeemParams,
    ) -> Result<ExecuteLpredeemResponse> {
        self.post("/v5/alpha/lp/redeem", &params).await
    }

    pub async fn execute_lpstake(
        &self,
        params: ExecuteLpstakeParams,
    ) -> Result<ExecuteLpstakeResponse> {
        self.post("/v5/alpha/lp/stake", &params).await
    }

    pub async fn execute_prediction_buy(
        &self,
        params: ExecutePredictionBuyParams,
    ) -> Result<ExecutePredictionBuyResponse> {
        self.post("/v5/alpha/prediction/buy", &params).await
    }

    pub async fn get_prediction_engine_status(&self) -> Result<GetPredictionEngineStatusResponse> {
        let params = vec![];
        self.get("/v5/alpha/prediction/engine-status", &params)
            .await
    }

    pub async fn get_prediction_event_detail(
        &self,
        params: GetPredictionEventDetailParams,
    ) -> Result<GetPredictionEventDetailResponse> {
        self.post("/v5/alpha/prediction/event-detail", &params)
            .await
    }

    pub async fn get_prediction_order_book(
        &self,
        params: GetPredictionOrderBookParams,
    ) -> Result<GetPredictionOrderBookResponse> {
        self.post("/v5/alpha/prediction/order-book", &params).await
    }

    pub async fn get_prediction_order_estimate(
        &self,
        params: GetPredictionOrderEstimateParams,
    ) -> Result<GetPredictionOrderEstimateResponse> {
        self.post("/v5/alpha/prediction/order-estimate", &params)
            .await
    }

    pub async fn get_prediction_order_list(
        &self,
        params: GetPredictionOrderListParams,
    ) -> Result<GetPredictionOrderListResponse> {
        self.post("/v5/alpha/prediction/order-list", &params).await
    }

    pub async fn get_prediction_pay_token_list(&self) -> Result<GetPredictionPayTokenListResponse> {
        let params = vec![];
        self.get("/v5/alpha/prediction/pay-token-list", &params)
            .await
    }

    pub async fn get_prediction_portfolio_summary(
        &self,
    ) -> Result<GetPredictionPortfolioSummaryResponse> {
        self.post(
            "/v5/alpha/prediction/portfolio-summary",
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn get_prediction_position_history(
        &self,
        params: GetPredictionPositionHistoryParams,
    ) -> Result<GetPredictionPositionHistoryResponse> {
        self.post("/v5/alpha/prediction/position-history", &params)
            .await
    }

    pub async fn get_prediction_position_list(
        &self,
        params: GetPredictionPositionListParams,
    ) -> Result<GetPredictionPositionListResponse> {
        self.post("/v5/alpha/prediction/position-list", &params)
            .await
    }

    pub async fn get_prediction_price_history(
        &self,
        params: GetPredictionPriceHistoryParams,
    ) -> Result<GetPredictionPriceHistoryResponse> {
        self.post("/v5/alpha/prediction/price-history", &params)
            .await
    }

    pub async fn execute_prediction_sell(
        &self,
        params: ExecutePredictionSellParams,
    ) -> Result<ExecutePredictionSellResponse> {
        self.post("/v5/alpha/prediction/sell", &params).await
    }

    pub async fn get_prediction_side_market_list(
        &self,
        params: GetPredictionSideMarketListParams,
    ) -> Result<GetPredictionSideMarketListResponse> {
        self.post("/v5/alpha/prediction/side-market-list", &params)
            .await
    }

    pub async fn get_prediction_group_stage_detail(
        &self,
        params: GetPredictionGroupStageDetailParams,
    ) -> Result<GetPredictionGroupStageDetailResponse> {
        self.post("/v5/alpha/prediction/sports/group-stage-detail", &params)
            .await
    }

    pub async fn get_prediction_match_list(
        &self,
        params: GetPredictionMatchListParams,
    ) -> Result<GetPredictionMatchListResponse> {
        self.post("/v5/alpha/prediction/sports/match-list", &params)
            .await
    }

    pub async fn get_prediction_timeline_stages(
        &self,
        event_type: Option<i64>,
    ) -> Result<GetPredictionTimelineStagesResponse> {
        let mut params = vec![];
        let event_type_string;
        if let Some(value) = event_type {
            event_type_string = value.to_string();
            params.push(("eventType", event_type_string.as_str()));
        }
        self.get("/v5/alpha/prediction/sports/timeline-stages", &params)
            .await
    }

    pub async fn get_prediction_token_price(
        &self,
        params: GetPredictionTokenPriceParams,
    ) -> Result<GetPredictionTokenPriceResponse> {
        self.post("/v5/alpha/prediction/token-price", &params).await
    }

    pub async fn get_asset_detail(
        &self,
        params: GetAssetDetailParams,
    ) -> Result<GetAssetDetailResponse> {
        self.post("/v5/alpha/trade/asset-detail", &params).await
    }

    pub async fn get_asset_list(&self) -> Result<GetAssetListResponse> {
        self.post("/v5/alpha/trade/asset-list", &serde_json::json!({}))
            .await
    }

    pub async fn get_biz_token_details(
        &self,
        params: GetBizTokenDetailsParams,
    ) -> Result<GetBizTokenDetailsResponse> {
        self.post("/v5/alpha/trade/biz-token-details", &params)
            .await
    }

    pub async fn get_biz_token_list(
        &self,
        params: GetBizTokenListParams,
    ) -> Result<GetBizTokenListResponse> {
        self.post("/v5/alpha/trade/biz-token-list", &params).await
    }

    pub async fn get_biz_token_price_list(
        &self,
        params: GetBizTokenPriceListParams,
    ) -> Result<GetBizTokenPriceListResponse> {
        self.post("/v5/alpha/trade/biz-token-price-list", &params)
            .await
    }

    pub async fn get_order_list(&self, params: GetOrderListParams) -> Result<GetOrderListResponse> {
        self.post("/v5/alpha/trade/order-list", &params).await
    }

    pub async fn get_pay_token_list(
        &self,
        params: GetPayTokenListParams,
    ) -> Result<GetPayTokenListResponse> {
        self.post("/v5/alpha/trade/pay-token-list", &params).await
    }

    pub async fn execute_purchase(
        &self,
        params: ExecutePurchaseParams,
    ) -> Result<ExecutePurchaseResponse> {
        self.post("/v5/alpha/trade/purchase", &params).await
    }

    pub async fn get_trade_quote(
        &self,
        params: GetTradeQuoteParams,
    ) -> Result<GetTradeQuoteResponse> {
        self.post("/v5/alpha/trade/quote", &params).await
    }

    pub async fn execute_redeem(
        &self,
        params: ExecuteRedeemParams,
    ) -> Result<ExecuteRedeemResponse> {
        self.post("/v5/alpha/trade/redeem", &params).await
    }
}
