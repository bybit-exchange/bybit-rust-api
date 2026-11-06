use crate::client::BybitClient;
use crate::error::Result;
use crate::models::spread_trade::*;
use crate::models::Side;

impl BybitClient {
    pub async fn get_spread_instruments_info(
        &self,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetSpreadInstrumentsInfoResponse> {
        let mut params = vec![];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        if let Some(value) = base_coin {
            params.push(("baseCoin", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get_public("/v5/spread/instrument", &params).await
    }

    pub async fn get_spread_orderbook(
        &self,
        symbol: &str,
        limit: Option<i64>,
    ) -> Result<GetSpreadOrderbookResponse> {
        let mut params = vec![("symbol", symbol)];
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        self.get_public("/v5/spread/orderbook", &params).await
    }

    pub async fn get_spread_recent_trades(
        &self,
        symbol: &str,
        limit: Option<i64>,
    ) -> Result<GetSpreadRecentTradesResponse> {
        let mut params = vec![("symbol", symbol)];
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        self.get_public("/v5/spread/recent-trade", &params).await
    }

    pub async fn get_spread_tickers(&self, symbol: &str) -> Result<GetSpreadTickersResponse> {
        let params = vec![("symbol", symbol)];
        self.get_public("/v5/spread/tickers", &params).await
    }

    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    pub async fn amend_spread_order(
        &self,
        params: AmendSpreadOrderParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/spread/order/amend", &params).await
    }

    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    pub async fn cancel_all_spread_orders(
        &self,
        params: CancelAllSpreadOrdersParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/spread/order/cancel-all", &params).await
    }

    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    pub async fn cancel_spread_order(
        &self,
        params: CancelSpreadOrderParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/spread/order/cancel", &params).await
    }

    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    pub async fn create_spread_order(
        &self,
        params: CreateSpreadOrderParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/spread/order/create", &params).await
    }

    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    pub async fn get_spread_open_orders(
        &self,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut params = vec![];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        if let Some(value) = base_coin {
            params.push(("baseCoin", value));
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
        self.get("/v5/spread/order/realtime", &params).await
    }

    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_spread_order_history(
        &self,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut params = vec![];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        if let Some(value) = base_coin {
            params.push(("baseCoin", value));
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
        self.get("/v5/spread/order/history", &params).await
    }

    // FIXME: OpenAPI response type object has no declared model; using serde_json::Value.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_spread_trade_history(
        &self,
        symbol: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<serde_json::Value> {
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
        self.get("/v5/spread/execution/list", &params).await
    }

    pub async fn get_spread_max_qty(
        &self,
        symbol: &str,
        side: Side,
        order_price: &str,
    ) -> Result<GetSpreadMaxQtyResponse> {
        let side_string = side.to_string();
        let params = vec![
            ("symbol", symbol),
            ("side", side_string.as_str()),
            ("orderPrice", order_price),
        ];
        self.get("/v5/spread/max-qty", &params).await
    }
}
