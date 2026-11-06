//! Trade API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::*;
use tracing::info;

impl BybitClient {
    /// Place an order.
    ///
    /// # Arguments
    /// * `params` - Order parameters
    ///
    /// # Example
    /// ```rust,no_run
    /// # use bybit_api::{BybitClient, Category, Side};
    /// # use bybit_api::trade::PlaceOrderParams;
    /// # async fn example() -> bybit_api::Result<()> {
    /// let client = BybitClient::testnet("key", "secret")?;
    /// let params = PlaceOrderParams::market(Category::Linear, "BTCUSDT", Side::Buy, "0.01");
    /// let result = client.place_order(params).await?;
    /// println!("Order ID: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_order(&self, params: PlaceOrderParams) -> Result<OrderResponse> {
        // Validate parameters before sending (fund safety)
        params.validate()?;

        info!(
            symbol = %params.symbol,
            side = ?params.side,
            order_type = ?params.order_type,
            qty = %params.qty,
            "Placing order"
        );

        self.post("/v5/order/create", &params).await
    }

    /// Amend an existing order.
    pub async fn amend_order(&self, params: AmendOrderParams) -> Result<OrderResponse> {
        info!(
            symbol = %params.symbol,
            order_id = ?params.order_id,
            "Amending order"
        );

        self.post("/v5/order/amend", &params).await
    }

    /// Cancel an order.
    pub async fn cancel_order(&self, params: CancelOrderParams) -> Result<OrderResponse> {
        info!(
            symbol = %params.symbol,
            order_id = ?params.order_id,
            "Cancelling order"
        );

        self.post("/v5/order/cancel", &params).await
    }

    /// Cancel all orders.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter (cancel all if None)
    pub async fn cancel_all_orders(
        &self,
        category: Category,
        symbol: Option<&str>,
    ) -> Result<CancelAllResponse> {
        let params = CancelAllOrdersParams {
            category,
            symbol: symbol.map(|s| s.to_string()),
            base_coin: None,
            settle_coin: None,
        };

        info!(category = ?category, symbol = ?symbol, "Cancelling all orders");

        self.post("/v5/order/cancel-all", &params).await
    }

    /// Place batch orders (up to 10 orders).
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `orders` - List of orders to place
    pub async fn place_batch_order(
        &self,
        category: Category,
        orders: Vec<PlaceOrderParams>,
    ) -> Result<BatchOrderResponse> {
        // Validate all orders
        for order in &orders {
            order.validate()?;
        }

        let request = BatchOrderRequest {
            category,
            request: orders,
        };

        info!(category = ?category, "Placing batch orders");

        self.post("/v5/order/create-batch", &request).await
    }

    /// Amend batch orders.
    pub async fn amend_batch_order(
        &self,
        category: Category,
        orders: Vec<AmendOrderParams>,
    ) -> Result<BatchOrderResponse> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct BatchAmendRequest {
            category: Category,
            request: Vec<AmendOrderParams>,
        }

        let request = BatchAmendRequest {
            category,
            request: orders,
        };

        self.post("/v5/order/amend-batch", &request).await
    }

    /// Cancel batch orders.
    pub async fn cancel_batch_order(
        &self,
        category: Category,
        orders: Vec<CancelOrderParams>,
    ) -> Result<BatchOrderResponse> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct BatchCancelRequest {
            category: Category,
            request: Vec<CancelOrderParams>,
        }

        let request = BatchCancelRequest {
            category,
            request: orders,
        };

        self.post("/v5/order/cancel-batch", &request).await
    }

    /// Get open orders (realtime).
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    pub async fn get_open_orders(
        &self,
        category: Category,
        symbol: Option<&str>,
    ) -> Result<OrdersList> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get("/v5/order/realtime", &params).await
    }

    /// Get order history.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    /// * `limit` - Optional limit (default 20)
    pub async fn get_order_history(
        &self,
        category: Category,
        symbol: Option<&str>,
        limit: Option<u32>,
    ) -> Result<OrdersList> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(20).to_string();
        let mut params = vec![
            ("category", cat_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get("/v5/order/history", &params).await
    }

    pub async fn dcp_set_timewindow(
        &self,
        params: DcpSetTimewindowParams,
    ) -> Result<DcpSetTimewindowResponse> {
        self.post("/v5/order/disconnected-cancel-all", &params)
            .await
    }

    pub async fn get_spot_borrow_quota(
        &self,
        category: Category,
        symbol: &str,
        side: Side,
    ) -> Result<GetSpotBorrowQuotaResponse> {
        let cat_str = category.to_string();
        let side_str = side.to_string();
        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("side", side_str.as_str()),
        ];
        self.get("/v5/order/spot-borrow-check", &params).await
    }

    pub async fn pre_check_order(
        &self,
        params: PreCheckOrderParams,
    ) -> Result<PreCheckOrderResponse> {
        self.post("/v5/order/pre-check", &params).await
    }
}
