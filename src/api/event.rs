use crate::client::BybitClient;
use crate::error::Result;
use crate::models::event::*;

impl BybitClient {
    #[allow(clippy::too_many_arguments)]
    pub async fn get_ec_order_history(
        &self,
        symbol: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetEcOrderHistoryResponse> {
        let mut params = vec![];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        if let Some(value) = order_id {
            params.push(("orderId", value));
        }
        if let Some(value) = order_link_id {
            params.push(("orderLinkId", value));
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
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/event/order-list", &params).await
    }

    pub async fn get_ec_active_orders(
        &self,
        symbol: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetEcActiveOrdersResponse> {
        let mut params = vec![];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        if let Some(value) = order_id {
            params.push(("orderId", value));
        }
        if let Some(value) = order_link_id {
            params.push(("orderLinkId", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/event/order-realtime", &params).await
    }

    pub async fn get_ec_position_info(
        &self,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetEcPositionInfoResponse> {
        let mut params = vec![];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        if let Some(value) = base_coin {
            params.push(("baseCoin", value));
        }
        if let Some(value) = settle_coin {
            params.push(("settleCoin", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/event/positions", &params).await
    }

    pub async fn get_ec_settlements(
        &self,
        symbol: &str,
        order_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetEcSettlementsResponse> {
        let mut params = vec![("symbol", symbol)];
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
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/event/settlements", &params).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_ec_trade_history(
        &self,
        symbol: &str,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetEcTradeHistoryResponse> {
        let mut params = vec![("symbol", symbol)];
        if let Some(value) = order_id {
            params.push(("orderId", value));
        }
        if let Some(value) = order_link_id {
            params.push(("orderLinkId", value));
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
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/event/trades", &params).await
    }
}
