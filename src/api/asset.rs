//! Asset API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::asset::*;
use crate::models::{AccountType, Category, ConvertAccountType, ConvertCoinSide};
use tracing::info;
use uuid::Uuid;

impl BybitClient {
    /// Get coin info.
    ///
    /// # Arguments
    /// * `coin` - Optional coin filter
    pub async fn get_coin_info(&self, coin: Option<&str>) -> Result<CoinInfoResponse> {
        let mut params = vec![];

        if let Some(c) = coin {
            params.push(("coin", c));
        }

        self.get("/v5/asset/coin/query-info", &params).await
    }

    /// Internal transfer between accounts.
    ///
    /// # Arguments
    /// * `coin` - Coin to transfer
    /// * `amount` - Amount to transfer
    /// * `from_account` - Source account type
    /// * `to_account` - Destination account type
    pub async fn internal_transfer(
        &self,
        coin: &str,
        amount: &str,
        from_account: &str,
        to_account: &str,
    ) -> Result<TransferResponse> {
        let params = InternalTransferParams {
            transfer_id: Uuid::new_v4().to_string(),
            coin: coin.to_string(),
            amount: amount.to_string(),
            from_account_type: from_account.to_string(),
            to_account_type: to_account.to_string(),
        };

        // Validate parameters
        params.validate()?;

        info!(
            coin = %coin,
            amount = %amount,
            from = %from_account,
            to = %to_account,
            "Internal transfer"
        );

        self.post("/v5/asset/transfer/inter-transfer", &params)
            .await
    }

    /// Get internal transfer list.
    ///
    /// # Arguments
    /// * `coin` - Optional coin filter
    /// * `limit` - Optional limit (default 20)
    pub async fn get_internal_transfer_list(
        &self,
        coin: Option<&str>,
        limit: Option<u32>,
    ) -> Result<TransferList> {
        let limit_str = limit.unwrap_or(20).to_string();
        let mut params = vec![("limit", limit_str.as_str())];

        if let Some(c) = coin {
            params.push(("coin", c));
        }

        self.get("/v5/asset/transfer/query-inter-transfer-list", &params)
            .await
    }

    /// Get deposit address.
    ///
    /// # Arguments
    /// * `coin` - Coin name
    /// * `chain_type` - Optional chain type
    pub async fn get_deposit_address(
        &self,
        coin: &str,
        chain_type: Option<&str>,
    ) -> Result<DepositAddressResponse> {
        let mut params = vec![("coin", coin)];

        if let Some(ct) = chain_type {
            params.push(("chainType", ct));
        }

        self.get("/v5/asset/deposit/query-address", &params).await
    }

    /// Get deposit records.
    ///
    /// # Arguments
    /// * `coin` - Optional coin filter
    /// * `limit` - Optional limit (default 50)
    pub async fn get_deposit_records(
        &self,
        coin: Option<&str>,
        limit: Option<u32>,
    ) -> Result<DepositRecords> {
        let limit_str = limit.unwrap_or(50).to_string();
        let mut params = vec![("limit", limit_str.as_str())];

        if let Some(c) = coin {
            params.push(("coin", c));
        }

        self.get("/v5/asset/deposit/query-record", &params).await
    }

    /// Withdraw funds (REQUIRES STRICT VALIDATION).
    ///
    /// # Arguments
    /// * `params` - Withdraw parameters
    ///
    /// # Safety
    /// This function validates all parameters before sending to prevent fund loss.
    pub async fn withdraw(&self, params: WithdrawParams) -> Result<WithdrawResponse> {
        // CRITICAL: Validate all parameters (fund safety)
        params.validate()?;

        info!(
            coin = %params.coin,
            chain = %params.chain,
            address = %params.address,
            amount = %params.amount,
            "Initiating withdrawal"
        );

        self.post("/v5/asset/withdraw/create", &params).await
    }

    /// Cancel a pending withdrawal.
    ///
    /// # Arguments
    /// * `withdraw_id` - Withdraw ID to cancel
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn cancel_withdraw(&self, withdraw_id: &str) -> Result<serde_json::Value> {
        let params = CancelWithdrawParams {
            id: withdraw_id.to_string(),
        };

        info!(withdraw_id = %withdraw_id, "Cancelling withdrawal");

        self.post("/v5/asset/withdraw/cancel", &params).await
    }

    /// Get withdraw records.
    ///
    /// # Arguments
    /// * `coin` - Optional coin filter
    /// * `limit` - Optional limit (default 50)
    pub async fn get_withdraw_records(
        &self,
        coin: Option<&str>,
        limit: Option<u32>,
    ) -> Result<WithdrawRecords> {
        let limit_str = limit.unwrap_or(50).to_string();
        let mut params = vec![("limit", limit_str.as_str())];

        if let Some(c) = coin {
            params.push(("coin", c));
        }

        self.get("/v5/asset/withdraw/query-record", &params).await
    }

    /// Get withdrawable amount.
    ///
    /// # Arguments
    /// * `coin` - Coin name
    pub async fn get_withdrawable_amount(&self, coin: &str) -> Result<WithdrawableAmount> {
        let params = vec![("coin", coin)];

        self.get("/v5/asset/withdraw/withdrawable-amount", &params)
            .await
    }
}

impl BybitClient {
    pub async fn set_default_deposit_to_account(
        &self,
        params: SetDefaultDepositToAccountParams,
    ) -> Result<SetDefaultDepositToAccountResponse> {
        self.post("/v5/asset/deposit/deposit-to-account", &params)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn query_internal_deposit_records(
        &self,
        tx_id: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        coin: Option<&str>,
        cursor: Option<&str>,
        limit: Option<i64>,
        status: Option<&str>,
    ) -> Result<QueryInternalDepositRecordsResponse> {
        let mut params = vec![];
        if let Some(value) = tx_id {
            params.push(("txID", value));
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
        if let Some(value) = coin {
            params.push(("coin", value));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        if let Some(value) = status {
            params.push(("status", value));
        }
        self.get("/v5/asset/deposit/query-internal-record", &params)
            .await
    }

    pub async fn query_sub_member_deposit_address(
        &self,
        coin: &str,
        chain_type: &str,
        sub_member_id: &str,
    ) -> Result<QuerySubMemberDepositAddressResponse> {
        let params = vec![
            ("coin", coin),
            ("chainType", chain_type),
            ("subMemberId", sub_member_id),
        ];
        self.get("/v5/asset/deposit/query-sub-member-address", &params)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn query_sub_member_deposit_records(
        &self,
        id: Option<&str>,
        tx_id: Option<&str>,
        sub_member_id: &str,
        coin: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<QuerySubMemberDepositRecordsResponse> {
        let mut params = vec![("subMemberId", sub_member_id)];
        if let Some(value) = id {
            params.push(("id", value));
        }
        if let Some(value) = tx_id {
            params.push(("txID", value));
        }
        if let Some(value) = coin {
            params.push(("coin", value));
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
        self.get("/v5/asset/deposit/query-sub-member-record", &params)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn account_coin_balance_query(
        &self,
        account_type: AccountType,
        coin: &str,
        member_id: Option<i64>,
        to_member_id: Option<i64>,
        to_account_type: Option<&str>,
        with_bonus: Option<i64>,
        with_transfer_safe_amount: Option<i64>,
        with_ltv_transfer_safe_amount: Option<i64>,
    ) -> Result<AccountCoinBalanceQueryResponse> {
        let account_type_string = account_type.to_string();
        let mut params = vec![
            ("accountType", account_type_string.as_str()),
            ("coin", coin),
        ];
        let member_id_string;
        if let Some(value) = member_id {
            member_id_string = value.to_string();
            params.push(("memberId", member_id_string.as_str()));
        }
        let to_member_id_string;
        if let Some(value) = to_member_id {
            to_member_id_string = value.to_string();
            params.push(("toMemberId", to_member_id_string.as_str()));
        }
        if let Some(value) = to_account_type {
            params.push(("toAccountType", value));
        }
        let with_bonus_string;
        if let Some(value) = with_bonus {
            with_bonus_string = value.to_string();
            params.push(("withBonus", with_bonus_string.as_str()));
        }
        let with_transfer_safe_amount_string;
        if let Some(value) = with_transfer_safe_amount {
            with_transfer_safe_amount_string = value.to_string();
            params.push((
                "withTransferSafeAmount",
                with_transfer_safe_amount_string.as_str(),
            ));
        }
        let with_ltv_transfer_safe_amount_string;
        if let Some(value) = with_ltv_transfer_safe_amount {
            with_ltv_transfer_safe_amount_string = value.to_string();
            params.push((
                "withLtvTransferSafeAmount",
                with_ltv_transfer_safe_amount_string.as_str(),
            ));
        }
        self.get("/v5/asset/transfer/query-account-coin-balance", &params)
            .await
    }

    pub async fn asset_info_query(
        &self,
        account_type: Option<AccountType>,
        coin: Option<&str>,
    ) -> Result<AssetInfoQueryResponse> {
        let mut params = vec![];
        let account_type_string;
        if let Some(value) = account_type {
            account_type_string = value.to_string();
            params.push(("accountType", account_type_string.as_str()));
        }
        if let Some(value) = coin {
            params.push(("coin", value));
        }
        self.get("/v5/asset/transfer/query-asset-info", &params)
            .await
    }

    pub async fn sub_member_list_query(&self) -> Result<SubMemberListQueryResponse> {
        let params = vec![];
        self.get("/v5/asset/transfer/query-sub-member-list", &params)
            .await
    }

    pub async fn transfer_coin_list_query(
        &self,
        from_account_type: &str,
        to_account_type: &str,
    ) -> Result<TransferCoinListQueryResponse> {
        let params = vec![
            ("fromAccountType", from_account_type),
            ("toAccountType", to_account_type),
        ];
        self.get("/v5/asset/transfer/query-transfer-coin-list", &params)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn universal_transfer_list_query(
        &self,
        transfer_id: Option<&str>,
        coin: Option<&str>,
        status: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<UniversalTransferListQueryResponse> {
        let mut params = vec![];
        if let Some(value) = transfer_id {
            params.push(("transferId", value));
        }
        if let Some(value) = coin {
            params.push(("coin", value));
        }
        if let Some(value) = status {
            params.push(("status", value));
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
        self.get("/v5/asset/transfer/query-universal-transfer-list", &params)
            .await
    }

    pub async fn transfer_sub_member_save(
        &self,
        params: TransferSubMemberSaveParams,
    ) -> Result<TransferSubMemberSaveResponse> {
        self.post("/v5/asset/transfer/save-transfer-sub-member", &params)
            .await
    }

    pub async fn universal_transfer(
        &self,
        params: UniversalTransferParams,
    ) -> Result<UniversalTransferResponse> {
        self.post("/v5/asset/transfer/universal-transfer", &params)
            .await
    }

    pub async fn query_withdraw_addresses(
        &self,
        coin: Option<&str>,
        chain: Option<&str>,
        address_type: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<QueryWithdrawAddressesResponse> {
        let mut params = vec![];
        if let Some(value) = coin {
            params.push(("coin", value));
        }
        if let Some(value) = chain {
            params.push(("chain", value));
        }
        let address_type_string;
        if let Some(value) = address_type {
            address_type_string = value.to_string();
            params.push(("addressType", address_type_string.as_str()));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/asset/withdraw/query-address", &params).await
    }

    pub async fn get_vasplist(&self) -> Result<GetVasplistResponse> {
        let params = vec![];
        self.get("/v5/asset/withdraw/vasp/list", &params).await
    }

    pub async fn get_asset_overview(
        &self,
        account_type: Option<&str>,
        member_id: Option<&str>,
        valuation_currency: Option<&str>,
    ) -> Result<GetAssetOverviewResponse> {
        let mut params = vec![];
        if let Some(value) = account_type {
            params.push(("accountType", value));
        }
        if let Some(value) = member_id {
            params.push(("memberId", value));
        }
        if let Some(value) = valuation_currency {
            params.push(("valuationCurrency", value));
        }
        self.get("/v5/asset/asset-overview", &params).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_delivery_record(
        &self,
        category: Category,
        symbol: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        exp_date: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetDeliveryRecordResponse> {
        let category_string = category.to_string();
        let mut params = vec![("category", category_string.as_str())];
        if let Some(value) = symbol {
            params.push(("symbol", value));
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
        if let Some(value) = exp_date {
            params.push(("expDate", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/asset/delivery-record", &params).await
    }

    pub async fn get_portfolio_margin(
        &self,
        base_coin: Option<&str>,
    ) -> Result<GetPortfolioMarginResponse> {
        let mut params = vec![];
        if let Some(value) = base_coin {
            params.push(("baseCoin", value));
        }
        self.get("/v5/asset/portfolio-margin", &params).await
    }

    pub async fn get_settlement_record(
        &self,
        category: Category,
        symbol: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetSettlementRecordResponse> {
        let category_string = category.to_string();
        let mut params = vec![("category", category_string.as_str())];
        if let Some(value) = symbol {
            params.push(("symbol", value));
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
        self.get("/v5/asset/settlement-record", &params).await
    }

    pub async fn get_total_members_assets(
        &self,
        coin: Option<&str>,
    ) -> Result<GetTotalMembersAssetsResponse> {
        let mut params = vec![];
        if let Some(value) = coin {
            params.push(("coin", value));
        }
        self.get("/v5/asset/total-members-assets", &params).await
    }

    pub async fn coin_list_query(
        &self,
        account_type: ConvertAccountType,
        side: Option<ConvertCoinSide>,
        coin: Option<&str>,
    ) -> Result<CoinListQueryResponse> {
        let account_type_string = account_type.to_string();
        let mut params = vec![("accountType", account_type_string.as_str())];
        let side_string;
        if let Some(value) = side {
            side_string = value.to_string();
            params.push(("side", side_string.as_str()));
        }
        if let Some(value) = coin {
            params.push(("coin", value));
        }
        self.get("/v5/asset/exchange/query-coin-list", &params)
            .await
    }

    pub async fn convert_execute(
        &self,
        params: ConvertExecuteParams,
    ) -> Result<ConvertExecuteResponse> {
        self.post("/v5/asset/exchange/convert-execute", &params)
            .await
    }

    pub async fn convert_history_query(
        &self,
        account_type: Option<ConvertAccountType>,
        index: Option<i64>,
        limit: Option<i64>,
    ) -> Result<ConvertHistoryQueryResponse> {
        let mut params = vec![];
        let account_type_string;
        if let Some(value) = account_type {
            account_type_string = value.to_string();
            params.push(("accountType", account_type_string.as_str()));
        }
        let index_string;
        if let Some(value) = index {
            index_string = value.to_string();
            params.push(("index", index_string.as_str()));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        self.get("/v5/asset/exchange/query-convert-history", &params)
            .await
    }

    pub async fn query_result(
        &self,
        quote_tx_id: &str,
        account_type: ConvertAccountType,
    ) -> Result<QueryResultResponse> {
        let account_type_string = account_type.to_string();
        let params = vec![
            ("quoteTxId", quote_tx_id),
            ("accountType", account_type_string.as_str()),
        ];
        self.get("/v5/asset/exchange/convert-result-query", &params)
            .await
    }

    pub async fn query_order_by_page(
        &self,
        cursor: Option<&str>,
        limit: Option<i64>,
        to_coin: Option<&str>,
        from_coin: Option<&str>,
    ) -> Result<QueryOrderByPageResponse> {
        let mut params = vec![];
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        if let Some(value) = to_coin {
            params.push(("toCoin", value));
        }
        if let Some(value) = from_coin {
            params.push(("fromCoin", value));
        }
        self.get("/v5/asset/exchange/order-record", &params).await
    }

    pub async fn quote_apply(&self, params: QuoteApplyParams) -> Result<QuoteApplyResponse> {
        self.post("/v5/asset/exchange/quote-apply", &params).await
    }

    pub async fn small_asset_convert(
        &self,
        params: SmallAssetConvertParams,
    ) -> Result<SmallAssetConvertResponse> {
        self.post("/v5/asset/covert/small-balance-execute", &params)
            .await
    }

    pub async fn query_small_asset_convert_order(
        &self,
        account_type: Option<ConvertAccountType>,
        quote_id: Option<&str>,
        cursor: Option<&str>,
        size: Option<&str>,
        start_time: Option<&str>,
        end_time: Option<&str>,
    ) -> Result<QuerySmallAssetConvertOrderResponse> {
        let mut params = vec![];
        let account_type_string;
        if let Some(value) = account_type {
            account_type_string = value.to_string();
            params.push(("accountType", account_type_string.as_str()));
        }
        if let Some(value) = quote_id {
            params.push(("quoteId", value));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        if let Some(value) = size {
            params.push(("size", value));
        }
        if let Some(value) = start_time {
            params.push(("startTime", value));
        }
        if let Some(value) = end_time {
            params.push(("endTime", value));
        }
        self.get("/v5/asset/covert/small-balance-history", &params)
            .await
    }

    pub async fn query_small_asset_list(
        &self,
        account_type: ConvertAccountType,
        from_coin: Option<&str>,
    ) -> Result<QuerySmallAssetListResponse> {
        let account_type_string = account_type.to_string();
        let mut params = vec![("accountType", account_type_string.as_str())];
        if let Some(value) = from_coin {
            params.push(("fromCoin", value));
        }
        self.get("/v5/asset/covert/small-balance-list", &params)
            .await
    }

    pub async fn small_asset_quote(
        &self,
        params: SmallAssetQuoteParams,
    ) -> Result<SmallAssetQuoteResponse> {
        self.post("/v5/asset/covert/get-quote", &params).await
    }
}
