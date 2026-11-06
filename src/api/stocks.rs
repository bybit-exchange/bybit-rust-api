use crate::client::BybitClient;
use crate::error::Result;
use crate::models::stocks::*;

impl BybitClient {
    pub async fn place_stocks_order(
        &self,
        params: PlaceStocksOrderParams,
    ) -> Result<PlaceStocksOrderResponse> {
        self.post("/v5/rwa/stocks/order", &params).await
    }

    pub async fn cancel_stocks_order(
        &self,
        params: CancelStocksOrderParams,
    ) -> Result<CancelStocksOrderResponse> {
        self.post("/v5/rwa/stocks/order/cancel", &params).await
    }

    pub async fn get_stocks_order_detail(
        &self,
        order_no: Option<&str>,
        request_id: Option<&str>,
    ) -> Result<GetStocksOrderDetailResponse> {
        let mut params = vec![];
        if let Some(value) = order_no {
            params.push(("orderNo", value));
        }
        if let Some(value) = request_id {
            params.push(("requestId", value));
        }
        self.get("/v5/rwa/stocks/order/detail", &params).await
    }

    pub async fn get_convert_list(&self, symbol: Option<&str>) -> Result<GetConvertListResponse> {
        let mut params = vec![];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        self.get("/v5/rwa/stocks/convert/list", &params).await
    }

    pub async fn submit_convert(
        &self,
        params: SubmitConvertParams,
    ) -> Result<SubmitConvertResponse> {
        self.post("/v5/rwa/stocks/convert/submit", &params).await
    }

    pub async fn get_convert_detail(
        &self,
        order_no: Option<&str>,
        request_id: Option<&str>,
    ) -> Result<GetConvertDetailResponse> {
        let mut params = vec![];
        if let Some(value) = order_no {
            params.push(("orderNo", value));
        }
        if let Some(value) = request_id {
            params.push(("requestId", value));
        }
        self.get("/v5/rwa/stocks/convert/detail", &params).await
    }

    pub async fn get_stocks_positions(
        &self,
        symbol: Option<&str>,
        show_zero: Option<bool>,
    ) -> Result<GetStocksPositionsResponse> {
        let mut params = vec![];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        let show_zero_string;
        if let Some(value) = show_zero {
            show_zero_string = value.to_string();
            params.push(("showZero", show_zero_string.as_str()));
        }
        self.get("/v5/rwa/stocks/positions", &params).await
    }

    pub async fn get_market_session(
        &self,
        symbol: Option<&str>,
    ) -> Result<GetMarketSessionResponse> {
        let mut params = vec![];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        self.get("/v5/rwa/stocks/market/session", &params).await
    }

    pub async fn get_multiplier_list(
        &self,
        symbol: Option<&str>,
        current: i64,
        page_size: i64,
    ) -> Result<GetMultiplierListResponse> {
        let current_string = current.to_string();
        let page_size_string = page_size.to_string();
        let mut params = vec![
            ("current", current_string.as_str()),
            ("pageSize", page_size_string.as_str()),
        ];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        self.get("/v5/rwa/stocks/market/getMultiplierList", &params)
            .await
    }
}
