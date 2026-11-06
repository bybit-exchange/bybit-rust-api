use crate::client::BybitClient;
use crate::error::Result;
use crate::models::pre_upgrade::*;
use crate::models::Category;

impl BybitClient {
    #[allow(clippy::too_many_arguments)]
    pub async fn get_pre_upgrade_transaction_log(
        &self,
        category: Category,
        base_coin: Option<&str>,
        type_: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetPreUpgradeTransactionLogResponse> {
        let category_string = category.to_string();
        let mut params = vec![("category", category_string.as_str())];
        if let Some(value) = base_coin {
            params.push(("baseCoin", value));
        }
        if let Some(value) = type_ {
            params.push(("type", value));
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
        self.get("/v5/pre-upgrade/account/transaction-log", &params)
            .await
    }

    pub async fn get_pre_upgrade_delivery_record(
        &self,
        category: Category,
        symbol: Option<&str>,
        exp_date: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetPreUpgradeDeliveryRecordResponse> {
        let category_string = category.to_string();
        let mut params = vec![("category", category_string.as_str())];
        if let Some(value) = symbol {
            params.push(("symbol", value));
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
        self.get("/v5/pre-upgrade/asset/delivery-record", &params)
            .await
    }

    pub async fn get_pre_upgrade_settlement_record(
        &self,
        category: Category,
        symbol: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetPreUpgradeSettlementRecordResponse> {
        let category_string = category.to_string();
        let mut params = vec![("category", category_string.as_str())];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/pre-upgrade/asset/settlement-record", &params)
            .await
    }

    pub async fn get_pre_upgrade_closed_pnl(
        &self,
        category: Category,
        symbol: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetPreUpgradeClosedPnlResponse> {
        let category_string = category.to_string();
        let mut params = vec![("category", category_string.as_str()), ("symbol", symbol)];
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
        self.get("/v5/pre-upgrade/position/closed-pnl", &params)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_pre_upgrade_execution_list(
        &self,
        category: Category,
        symbol: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        base_coin: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        exec_type: Option<&str>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetPreUpgradeExecutionListResponse> {
        let category_string = category.to_string();
        let mut params = vec![("category", category_string.as_str())];
        if let Some(value) = symbol {
            params.push(("symbol", value));
        }
        if let Some(value) = order_id {
            params.push(("orderId", value));
        }
        if let Some(value) = order_link_id {
            params.push(("orderLinkId", value));
        }
        if let Some(value) = base_coin {
            params.push(("baseCoin", value));
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
        if let Some(value) = exec_type {
            params.push(("execType", value));
        }
        let limit_string;
        if let Some(value) = limit {
            limit_string = value.to_string();
            params.push(("limit", limit_string.as_str()));
        }
        if let Some(value) = cursor {
            params.push(("cursor", value));
        }
        self.get("/v5/pre-upgrade/execution/list", &params).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_pre_upgrade_order_history(
        &self,
        category: Category,
        symbol: Option<&str>,
        base_coin: Option<&str>,
        order_id: Option<&str>,
        order_link_id: Option<&str>,
        order_filter: Option<&str>,
        order_status: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<GetPreUpgradeOrderHistoryResponse> {
        let category_string = category.to_string();
        let mut params = vec![("category", category_string.as_str())];
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
        if let Some(value) = order_filter {
            params.push(("orderFilter", value));
        }
        if let Some(value) = order_status {
            params.push(("orderStatus", value));
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
        self.get("/v5/pre-upgrade/order/history", &params).await
    }
}
