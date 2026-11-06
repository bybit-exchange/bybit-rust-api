use crate::client::BybitClient;
use crate::error::Result;
use crate::models::strategy::*;
use crate::models::Category;

impl BybitClient {
    pub async fn create_strategy(
        &self,
        params: serde_json::Value,
    ) -> Result<CreateStrategyResponse> {
        self.post("/v5/strategy/create", &params).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn query_strategy_order_list(
        &self,
        strategy_id: &str,
        status: Option<&str>,
        symbol: Option<&str>,
        begin_time_e0: Option<i64>,
        end_time_e0: Option<i64>,
        page_size: Option<i64>,
        cursor: Option<&str>,
        strategy_type: Option<&str>,
    ) -> Result<QueryStrategyOrderListResponse> {
        let mut params = vec![("strategyId", strategy_id)];
        if let Some(value) = status {
            params.push(("status", value));
        }
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        let begin_time_e0_string;
        if let Some(value) = begin_time_e0 {
            begin_time_e0_string = value.to_string();
            params.push(("BeginTimeE0", begin_time_e0_string.as_str()));
        }
        let end_time_e0_string;
        if let Some(value) = end_time_e0 {
            end_time_e0_string = value.to_string();
            params.push(("EndTimeE0", end_time_e0_string.as_str()));
        }
        let page_size_string;
        if let Some(value) = page_size {
            page_size_string = value.to_string();
            params.push(("pageSize", page_size_string.as_str()));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        if let Some(value) = strategy_type {
            params.push(("StrategyType", value));
        }
        self.get("/v5/strategy/order-list", &params).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn query_strategy_list(
        &self,
        strategy_id: Option<&str>,
        status: Option<&str>,
        symbol: Option<&str>,
        category: Option<Category>,
        strategy_type: Option<&str>,
        begin_time_e0: Option<i64>,
        end_time_e0: Option<i64>,
        page_size: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<QueryStrategyListResponse> {
        let mut params = vec![];
        if let Some(value) = strategy_id {
            params.push(("strategyId", value));
        }
        if let Some(value) = status {
            params.push(("status", value));
        }
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        let category_string;
        if let Some(value) = category {
            category_string = value.to_string();
            params.push(("category", category_string.as_str()));
        }
        if let Some(value) = strategy_type {
            params.push(("strategyType", value));
        }
        let begin_time_e0_string;
        if let Some(value) = begin_time_e0 {
            begin_time_e0_string = value.to_string();
            params.push(("beginTimeE0", begin_time_e0_string.as_str()));
        }
        let end_time_e0_string;
        if let Some(value) = end_time_e0 {
            end_time_e0_string = value.to_string();
            params.push(("endTimeE0", end_time_e0_string.as_str()));
        }
        let page_size_string;
        if let Some(value) = page_size {
            page_size_string = value.to_string();
            params.push(("pageSize", page_size_string.as_str()));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/strategy/list", &params).await
    }

    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    pub async fn stop_strategy(&self) -> Result<serde_json::Value> {
        self.post("/v5/strategy/stop", &serde_json::json!({})).await
    }
}
