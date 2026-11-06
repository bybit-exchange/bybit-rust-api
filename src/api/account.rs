//! Account API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::*;

impl BybitClient {
    /// Get wallet balance.
    ///
    /// # Arguments
    /// * `account_type` - Account type (UNIFIED, CONTRACT, etc.)
    pub async fn get_wallet_balance(&self, account_type: AccountType) -> Result<WalletBalance> {
        let account_type_str = account_type.to_string();
        let params = vec![("accountType", account_type_str.as_str())];

        self.get("/v5/account/wallet-balance", &params).await
    }

    /// Get account info.
    pub async fn get_account_info(&self) -> Result<AccountInfo> {
        self.get("/v5/account/info", &[]).await
    }

    /// Get fee rate.
    ///
    /// # Arguments
    /// * `category` - Product category
    /// * `symbol` - Optional symbol filter
    pub async fn get_fee_rate(&self, category: Category, symbol: Option<&str>) -> Result<FeeRates> {
        let cat_str = category.to_string();
        let mut params = vec![("category", cat_str.as_str())];

        if let Some(s) = symbol {
            params.push(("symbol", s));
        }

        self.get("/v5/account/fee-rate", &params).await
    }

    /// Get transaction log.
    ///
    /// # Arguments
    /// * `category` - Optional category filter
    /// * `limit` - Optional limit (default 20)
    pub async fn get_transaction_log(
        &self,
        category: Option<Category>,
        limit: Option<u32>,
    ) -> Result<TransactionLogs> {
        let limit_str = limit.unwrap_or(20).to_string();
        let mut params = vec![("limit", limit_str.as_str())];

        let cat_str;
        if let Some(c) = category {
            cat_str = c.to_string();
            params.push(("category", cat_str.as_str()));
        }

        self.get("/v5/account/transaction-log", &params).await
    }

    /// Set margin mode.
    ///
    /// # Arguments
    /// * `margin_mode` - Margin mode (REGULAR_MARGIN, PORTFOLIO_MARGIN)
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn set_margin_mode(&self, margin_mode: MarginMode) -> Result<serde_json::Value> {
        let params = SetMarginModeParams {
            set_margin_mode: margin_mode.to_string(),
        };

        self.post("/v5/account/set-margin-mode", &params).await
    }

    /// Get collateral info.
    ///
    /// # Arguments
    /// * `currency` - Optional currency filter
    pub async fn get_collateral_info(&self, currency: Option<&str>) -> Result<CollateralInfo> {
        let mut params = vec![];

        if let Some(c) = currency {
            params.push(("currency", c));
        }

        self.get("/v5/account/collateral-info", &params).await
    }

    /// Get borrow history.
    ///
    /// # Arguments
    /// * `currency` - Optional currency filter
    /// * `limit` - Optional limit (default 20)
    pub async fn get_borrow_history(
        &self,
        currency: Option<&str>,
        limit: Option<u32>,
    ) -> Result<BorrowHistory> {
        let limit_str = limit.unwrap_or(20).to_string();
        let mut params = vec![("limit", limit_str.as_str())];

        if let Some(c) = currency {
            params.push(("currency", c));
        }

        self.get("/v5/account/borrow-history", &params).await
    }

    pub async fn batch_set_collateral(
        &self,
        params: BatchSetCollateralParams,
    ) -> Result<BatchSetCollateralResponse> {
        self.post("/v5/account/set-collateral-switch-batch", &params)
            .await
    }

    pub async fn get_account_instruments(
        &self,
        category: Category,
        symbol: Option<&str>,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<GetAccountInstrumentsResponse> {
        let cat_str = category.to_string();
        let limit_str;
        let mut params = vec![("category", cat_str.as_str())];
        if let Some(s) = symbol {
            params.push(("symbol", s));
        }
        if let Some(l) = limit {
            limit_str = l.to_string();
            params.push(("limit", limit_str.as_str()));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c));
        }
        self.get("/v5/account/instruments-info", &params).await
    }

    pub async fn get_coin_greeks(&self, base_coin: Option<&str>) -> Result<GetCoinGreeksResponse> {
        let mut params = vec![];
        if let Some(b) = base_coin {
            params.push(("baseCoin", b));
        }
        self.get("/v5/asset/coin-greeks", &params).await
    }

    pub async fn get_dcp_info(&self) -> Result<GetDcpInfoResponse> {
        self.get("/v5/account/query-dcp-info", &[]).await
    }

    pub async fn get_group_fee_rate(
        &self,
        product_type: &str,
        group_id: Option<&str>,
    ) -> Result<GetGroupFeeRateResponse> {
        let mut params = vec![("productType", product_type)];
        if let Some(g) = group_id {
            params.push(("groupId", g));
        }
        self.get_public("/v5/market/fee-group-info", &params).await
    }

    pub async fn get_mmp_state(&self, base_coin: &str) -> Result<GetMmpStateResponse> {
        let params = vec![("baseCoin", base_coin)];
        self.get("/v5/account/mmp-state", &params).await
    }

    pub async fn get_smp_group(&self) -> Result<GetSmpGroupResponse> {
        self.get("/v5/account/smp-group", &[]).await
    }

    pub async fn get_transferable_amount(
        &self,
        coin_name: &str,
    ) -> Result<GetTransferableAmountResponse> {
        let params = vec![("coinName", coin_name)];
        self.get("/v5/account/withdrawal", &params).await
    }

    pub async fn get_user_settings(&self) -> Result<GetUserSettingsResponse> {
        self.get("/v5/account/user-setting-config", &[]).await
    }

    pub async fn manual_borrow(&self, params: ManualBorrowParams) -> Result<ManualBorrowResponse> {
        self.post("/v5/account/borrow", &params).await
    }

    pub async fn manual_repay(&self, params: ManualRepayParams) -> Result<ManualRepayResponse> {
        self.post("/v5/account/repay", &params).await
    }

    pub async fn no_convert_repay(
        &self,
        params: NoConvertRepayParams,
    ) -> Result<NoConvertRepayResponse> {
        self.post("/v5/account/no-convert-repay", &params).await
    }

    pub async fn one_click_repay(
        &self,
        params: OneClickRepayParams,
    ) -> Result<OneClickRepayResponse> {
        self.post("/v5/account/quick-repayment", &params).await
    }

    pub async fn reset_mmp(&self, params: ResetMmpParams) -> Result<ResetMmpResponse> {
        self.post("/v5/account/mmp-reset", &params).await
    }

    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn set_collateral_coin(
        &self,
        params: SetCollateralCoinParams,
    ) -> Result<serde_json::Value> {
        self.post("/v5/account/set-collateral-switch", &params)
            .await
    }

    pub async fn set_mmp(&self, params: SetMmpParams) -> Result<SetMmpResponse> {
        self.post("/v5/account/mmp-modify", &params).await
    }

    pub async fn set_price_limit(
        &self,
        params: SetPriceLimitParams,
    ) -> Result<SetPriceLimitResponse> {
        self.post("/v5/account/set-limit-px-action", &params).await
    }

    pub async fn set_spot_hedging(
        &self,
        params: SetSpotHedgingParams,
    ) -> Result<SetSpotHedgingResponse> {
        self.post("/v5/account/set-hedging-mode", &params).await
    }

    pub async fn upgrade_to_uta_pro(&self) -> Result<UpgradeToUtaProResponse> {
        self.post("/v5/account/upgrade-to-uta", &serde_json::json!({}))
            .await
    }
}

impl BybitClient {
    pub async fn demo_apply_money(
        &self,
        params: DemoApplyMoneyParams,
    ) -> Result<DemoApplyMoneyResponse> {
        self.post("/v5/account/demo-apply-money", &params).await
    }

    pub async fn get_option_asset_info(&self) -> Result<GetOptionAssetInfoResponse> {
        let params = vec![];
        self.get("/v5/account/option-asset-info", &params).await
    }

    pub async fn get_pay_info(&self, coin: Option<&str>) -> Result<GetPayInfoResponse> {
        let mut params = vec![];
        if let Some(value) = coin {
            params.push(("coin", value));
        }
        self.get("/v5/account/pay-info", &params).await
    }

    pub async fn get_trade_info_for_analysis(
        &self,
        symbol: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<GetTradeInfoForAnalysisResponse> {
        let mut params = vec![("symbol", symbol)];
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
        self.get("/v5/account/trade-info-for-analysis", &params)
            .await
    }
}
