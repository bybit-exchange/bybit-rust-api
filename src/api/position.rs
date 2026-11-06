//! Position API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::Category;
use crate::models::*;

impl BybitClient {
    /// Get position list.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    /// * `settle_coin` - Optional settle coin filter (e.g., "USDT")
    pub async fn get_positions(
        &self,
        category: Category,
        symbol: Option<&str>,
        settle_coin: Option<&str>,
    ) -> Result<PositionList> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }
        if let Some(sc) = settle_coin {
            params.push(("settleCoin", sc));
        }

        self.get("/v5/position/list", &params).await
    }

    /// Set leverage.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `buy_leverage` - Buy leverage
    /// * `sell_leverage` - Sell leverage
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn set_leverage(
        &self,
        category: Category,
        symbol: &str,
        buy_leverage: &str,
        sell_leverage: &str,
    ) -> Result<serde_json::Value> {
        let params = SetLeverageParams {
            category,
            symbol: symbol.to_string(),
            buy_leverage: buy_leverage.to_string(),
            sell_leverage: sell_leverage.to_string(),
        };

        self.post("/v5/position/set-leverage", &params).await
    }

    /// Set trading stop (TP/SL).
    ///
    /// # Arguments
    /// * `params` - Trading stop parameters
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn set_trading_stop(&self, params: TradingStopParams) -> Result<serde_json::Value> {
        self.post("/v5/position/trading-stop", &params).await
    }

    /// Switch position mode.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `mode` - Position mode (0=merged, 3=both sides)
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn switch_position_mode(
        &self,
        category: Category,
        mode: PositionMode,
    ) -> Result<serde_json::Value> {
        let params = SwitchPositionModeParams {
            category,
            symbol: None,
            coin: None,
            mode: mode as i32,
        };

        self.post("/v5/position/switch-mode", &params).await
    }

    /// Set risk limit.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `risk_id` - Risk limit ID
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn set_risk_limit(
        &self,
        category: Category,
        symbol: &str,
        risk_id: i32,
    ) -> Result<serde_json::Value> {
        let params = SetRiskLimitParams {
            category,
            symbol: symbol.to_string(),
            risk_id,
            position_idx: None,
        };

        self.post("/v5/position/set-risk-limit", &params).await
    }

    /// Add or reduce margin.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Symbol name
    /// * `margin` - Margin amount (positive to add, negative to reduce)
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn add_margin(
        &self,
        category: Category,
        symbol: &str,
        margin: &str,
    ) -> Result<serde_json::Value> {
        let params = AddMarginParams {
            category,
            symbol: symbol.to_string(),
            margin: margin.to_string(),
            position_idx: None,
        };

        self.post("/v5/position/add-margin", &params).await
    }

    /// Get closed PnL history.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    /// * `limit` - Optional limit (default 20)
    pub async fn get_closed_pnl(
        &self,
        category: Category,
        symbol: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ClosedPnlList> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(20).to_string();
        let mut params = vec![
            ("category", cat_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get("/v5/position/closed-pnl", &params).await
    }

    /// Get execution list (trade history).
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    /// * `limit` - Optional limit (default 50)
    pub async fn get_executions(
        &self,
        category: Category,
        symbol: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ExecutionList> {
        let cat_str = category.to_string();
        let limit_str = limit.unwrap_or(50).to_string();
        let mut params = vec![
            ("category", cat_str.as_str()),
            ("limit", limit_str.as_str()),
        ];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get("/v5/execution/list", &params).await
    }

    pub async fn confirm_new_risk_limit(
        &self,
        params: ConfirmNewRiskLimitParams,
    ) -> Result<ConfirmNewRiskLimitResponse> {
        self.post("/v5/position/confirm-pending-mmr", &params).await
    }

    pub async fn get_close_position(
        &self,
        category: Category,
        symbol: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetClosePositionResponse> {
        let cat_str = category.to_string();
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let limit_str = limit.map(|v| v.to_string());
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(s) = symbol {
            params.push(("symbol", s));
        }
        if let Some(ref st) = start_time_str {
            params.push(("startTime", st.as_str()));
        }
        if let Some(ref et) = end_time_str {
            params.push(("endTime", et.as_str()));
        }
        if let Some(ref l) = limit_str {
            params.push(("limit", l.as_str()));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c));
        }
        self.get("/v5/position/get-closed-positions", &params).await
    }

    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn get_move_position_history(
        &self,
        category: Option<Category>,
        symbol: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status: Option<&str>,
        block_trade_id: Option<&str>,
        limit: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<GetMovePositionHistoryResponse> {
        let cat_str = category.map(|c| c.to_string());
        let start_time_str = start_time.map(|v| v.to_string());
        let end_time_str = end_time.map(|v| v.to_string());
        let mut params: Vec<(&str, &str)> = Vec::new();
        if let Some(ref c) = cat_str {
            params.push(("category", c.as_str()));
        }
        if let Some(s) = symbol {
            params.push(("symbol", s));
        }
        if let Some(ref st) = start_time_str {
            params.push(("startTime", st.as_str()));
        }
        if let Some(ref et) = end_time_str {
            params.push(("endTime", et.as_str()));
        }
        if let Some(s) = status {
            params.push(("status", s));
        }
        if let Some(b) = block_trade_id {
            params.push(("blockTradeId", b));
        }
        if let Some(l) = limit {
            params.push(("limit", l));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c));
        }
        self.get("/v5/position/move-history", &params).await
    }

    pub async fn move_position(&self, params: MovePositionParams) -> Result<MovePositionResponse> {
        self.post("/v5/position/move-positions", &params).await
    }

    pub async fn set_auto_add_margin(
        &self,
        params: SetAutoAddMarginParams,
    ) -> Result<SetAutoAddMarginResponse> {
        self.post("/v5/position/set-auto-add-margin", &params).await
    }
}

impl BybitClient {
    pub async fn get_position_symbol_info(
        &self,
        category: Category,
        symbol: Option<&str>,
    ) -> Result<GetPositionSymbolInfoResponse> {
        let category_string = category.to_string();
        let mut params = vec![("category", category_string.as_str())];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        self.get("/v5/position/symbol-info", &params).await
    }
}
