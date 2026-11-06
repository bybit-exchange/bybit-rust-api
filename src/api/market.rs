//! Market data API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::*;

impl BybitClient {
    /// Get server time.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use bybit_api::BybitClient;
    /// # async fn example() -> bybit_api::Result<()> {
    /// let client = BybitClient::testnet("key", "secret")?;
    /// let time = client.get_server_time().await?;
    /// println!("Server time: {}", time.time_second);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        self.get_public("/v5/market/time", &[]).await
    }

    /// Get instruments info.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    ///
    /// # Example
    /// ```rust,no_run
    /// # use bybit_api::{BybitClient, Category};
    /// # async fn example() -> bybit_api::Result<()> {
    /// let client = BybitClient::testnet("key", "secret")?;
    /// let info = client.get_instruments_info(Category::Linear, Some("BTCUSDT")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_instruments_info(
        &self,
        category: Category,
        symbol: Option<&str>,
    ) -> Result<InstrumentsInfo> {
        let mut params = vec![("category", category.to_string())];

        let symbol_str;
        if let Some(s) = symbol {
            symbol_str = s.to_string();
            params.push(("symbol", symbol_str.clone()));
        }

        let params_ref: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.get_public("/v5/market/instruments-info", &params_ref)
            .await
    }

    /// Get orderbook.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `limit` - Optional depth limit (1-500, default 25)
    pub async fn get_orderbook(
        &self,
        category: Category,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<Orderbook> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(25).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/orderbook", &params).await
    }

    /// Get tickers.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter (returns all if None)
    pub async fn get_tickers(&self, category: Category, symbol: Option<&str>) -> Result<Tickers> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get_public("/v5/market/tickers", &params).await
    }

    /// Get klines (candlestick data).
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `interval` - Kline interval
    /// * `limit` - Optional limit (1-1000, default 200)
    pub async fn get_klines(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        limit: Option<u32>,
    ) -> Result<Klines> {
        let cat_str = category.to_string();
        let interval_str = interval.to_string();
        let limit_str = limit.unwrap_or(200).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("interval", interval_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/kline", &params).await
    }

    /// Get funding rate history.
    ///
    /// # Arguments
    /// * `category` - Product category (linear or inverse)
    /// * `symbol` - Symbol name
    /// * `limit` - Optional limit (default 200)
    pub async fn get_funding_history(
        &self,
        category: Category,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<FundingHistory> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(200).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/funding/history", &params).await
    }

    /// Get recent trades.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `limit` - Optional limit (1-1000, default 500)
    pub async fn get_recent_trades(
        &self,
        category: Category,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<RecentTrades> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(500).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/recent-trade", &params).await
    }

    /// Get open interest.
    ///
    /// # Arguments
    /// * `category` - Product category (linear or inverse)
    /// * `symbol` - Symbol name
    /// * `interval_time` - Interval (5min, 15min, 30min, 1h, 4h, 1d)
    /// * `limit` - Optional limit (default 50)
    pub async fn get_open_interest(
        &self,
        category: Category,
        symbol: &str,
        interval_time: &str,
        limit: Option<u32>,
    ) -> Result<OpenInterest> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(50).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("intervalTime", interval_time),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/open-interest", &params).await
    }

    /// Get risk limit info.
    ///
    /// # Arguments
    /// * `category` - Product category (linear or inverse)
    /// * `symbol` - Optional symbol filter
    pub async fn get_risk_limit(
        &self,
        category: Category,
        symbol: Option<&str>,
    ) -> Result<RiskLimits> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get_public("/v5/market/risk-limit", &params).await
    }

    /// Get mark price kline.
    pub async fn get_mark_price_kline(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        limit: Option<u32>,
    ) -> Result<Klines> {
        let cat_str = category.to_string();
        let interval_str = interval.to_string();
        let limit_str = limit.unwrap_or(200).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("interval", interval_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/mark-price-kline", &params)
            .await
    }

    /// Get index price kline.
    pub async fn get_index_price_kline(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        limit: Option<u32>,
    ) -> Result<Klines> {
        let cat_str = category.to_string();
        let interval_str = interval.to_string();
        let limit_str = limit.unwrap_or(200).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("interval", interval_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/index-price-kline", &params)
            .await
    }

    /// Get premium index price kline.
    pub async fn get_premium_index_price_kline(
        &self,
        category: Category,
        symbol: &str,
        interval: Interval,
        limit: Option<u32>,
    ) -> Result<Klines> {
        let cat_str = category.to_string();
        let interval_str = interval.to_string();
        let limit_str = limit.unwrap_or(200).to_string();

        let params = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("interval", interval_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        self.get_public("/v5/market/premium-index-price-kline", &params)
            .await
    }

    pub async fn get_adl_alert(&self, symbol: Option<&str>) -> Result<GetAdlAlertResponse> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = symbol {
            params.push(("symbol", s));
        }
        self.get_public("/v5/market/adlAlert", &params).await
    }

    pub async fn get_delivery_price(
        &self,
        category: Category,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        settle_coin: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<GetDeliveryPriceResponse> {
        let cat_str = category.to_string();
        let limit_str = limit.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = vec![("category", cat_str.as_str())];
        if let Some(s) = symbol {
            params.push(("symbol", s));
        }
        if let Some(s) = base_coin {
            params.push(("baseCoin", s));
        }
        if let Some(s) = settle_coin {
            params.push(("settleCoin", s));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        if let Some(s) = cursor {
            params.push(("cursor", s));
        }
        self.get_public("/v5/market/delivery-price", &params).await
    }

    pub async fn get_historical_volatility(
        &self,
        category: Category,
        base_coin: Option<&str>,
        quote_coin: Option<&str>,
        period: Option<u32>,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<GetHistoricalVolatilityResponse> {
        let cat_str = category.to_string();
        let period_str = period.map(|v| v.to_string());
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = vec![("category", cat_str.as_str())];
        if let Some(s) = base_coin {
            params.push(("baseCoin", s));
        }
        if let Some(s) = quote_coin {
            params.push(("quoteCoin", s));
        }
        if let Some(ref s) = period_str {
            params.push(("period", s.as_str()));
        }
        if let Some(ref s) = start_time_str {
            params.push(("startTime", s.as_str()));
        }
        if let Some(ref s) = end_time_str {
            params.push(("endTime", s.as_str()));
        }
        self.get_public("/v5/market/historical-volatility", &params)
            .await
    }

    pub async fn get_index_price_components(
        &self,
        index_name: &str,
    ) -> Result<GetIndexPriceComponentsResponse> {
        let params: Vec<(&str, &str)> = vec![("indexName", index_name)];
        self.get_public("/v5/market/index-price-components", &params)
            .await
    }

    pub async fn get_insurance_pool(&self, coin: Option<&str>) -> Result<GetInsurancePoolResponse> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(s) = coin {
            params.push(("coin", s));
        }
        self.get_public("/v5/market/insurance", &params).await
    }

    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn get_long_short_ratio(
        &self,
        category: Category,
        symbol: &str,
        period: &str,
        start_time: Option<&str>,
        end_time: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<GetLongShortRatioResponse> {
        let cat_str = category.to_string();
        let limit_str = limit.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = vec![
            ("category", cat_str.as_str()),
            ("symbol", symbol),
            ("period", period),
        ];
        if let Some(s) = start_time {
            params.push(("startTime", s));
        }
        if let Some(s) = end_time {
            params.push(("endTime", s));
        }
        if let Some(ref s) = limit_str {
            params.push(("limit", s.as_str()));
        }
        if let Some(s) = cursor {
            params.push(("cursor", s));
        }
        self.get_public("/v5/market/account-ratio", &params).await
    }

    pub async fn get_new_delivery_price(
        &self,
        category: Category,
        base_coin: &str,
        settle_coin: Option<&str>,
    ) -> Result<GetNewDeliveryPriceResponse> {
        let cat_str = category.to_string();
        let mut params: Vec<(&str, &str)> =
            vec![("category", cat_str.as_str()), ("baseCoin", base_coin)];
        if let Some(s) = settle_coin {
            params.push(("settleCoin", s));
        }
        self.get_public("/v5/market/new-delivery-price", &params)
            .await
    }

    pub async fn get_order_price_limit(
        &self,
        category: Option<Category>,
        symbol: &str,
    ) -> Result<GetOrderPriceLimitResponse> {
        let cat_str = category.map(|c| c.to_string());
        let mut params: Vec<(&str, &str)> = vec![("symbol", symbol)];
        if let Some(ref s) = cat_str {
            params.push(("category", s.as_str()));
        }
        self.get_public("/v5/market/price-limit", &params).await
    }

    pub async fn get_rpi_orderbook(
        &self,
        category: Option<Category>,
        symbol: &str,
        limit: u32,
    ) -> Result<GetRpiOrderbookResponse> {
        let cat_str = category.map(|c| c.to_string());
        let limit_str = limit.to_string();
        let mut params: Vec<(&str, &str)> = vec![("symbol", symbol), ("limit", limit_str.as_str())];
        if let Some(ref s) = cat_str {
            params.push(("category", s.as_str()));
        }
        self.get_public("/v5/market/rpi_orderbook", &params).await
    }
}

impl BybitClient {
    pub async fn get_option_base_coins(
        &self,
        underlying_type: Option<i64>,
    ) -> Result<GetOptionBaseCoinsResponse> {
        let mut params = vec![];
        let underlying_type_string;
        if let Some(value) = underlying_type {
            underlying_type_string = value.to_string();
            params.push(("underlyingType", underlying_type_string.as_str()));
        }
        self.get_public("/v5/market/option-base-coins", &params)
            .await
    }
}
