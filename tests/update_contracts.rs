use bybit_api::alpha::{
    ExecutePredictionBuyParams, ExecutePredictionSellParams, GetBizTokenListResponse,
    GetLporderListParams, GetPredictionGroupStageDetailParams, GetPredictionOrderEstimateParams,
    GetPredictionOrderListParams, GetPredictionPriceHistoryParams,
    GetPredictionPriceHistoryResponse, GetPredictionSideMarketListParams,
};
use bybit_api::asset::QueryOrderByPageResponse;
use serde_json::json;

#[test]
fn lp_order_filters_accept_integer_order_types() {
    for order_type in [0, 1, 2] {
        let params: GetLporderListParams =
            serde_json::from_value(json!({"orderType": order_type})).unwrap();
        assert_eq!(
            serde_json::to_value(params).unwrap()["orderType"],
            order_type
        );
    }
}

#[test]
fn prediction_orders_use_numeric_fok_order_type() {
    let buy = json!({
        "tokenId": "token_yes_123", "amount": "10", "payTokenCode": "USDC",
        "orderType": 1, "slippage": "0.01", "eventId": "event-1"
    });
    let params: ExecutePredictionBuyParams = serde_json::from_value(buy.clone()).unwrap();
    assert_eq!(serde_json::to_value(params).unwrap(), buy);

    let sell: ExecutePredictionSellParams = serde_json::from_value(json!({
        "tokenId": "token_yes_123", "size": "10", "orderType": 1,
        "slippage": "0.01", "eventId": "event-1"
    }))
    .unwrap();
    assert_eq!(serde_json::to_value(sell).unwrap()["orderType"], 1);
}

#[test]
fn prediction_estimates_and_order_filters_use_numeric_sides() {
    for side in [1, 2] {
        let estimate: GetPredictionOrderEstimateParams = serde_json::from_value(json!({
            "tokenId": "token_yes_123", "side": side, "eventId": "event-1",
            "amount": "10", "orderType": 1
        }))
        .unwrap();
        let value = serde_json::to_value(estimate).unwrap();
        assert_eq!(value["side"], side);
        assert_eq!(value["orderType"], 1);

        let filter: GetPredictionOrderListParams =
            serde_json::from_value(json!({"side": side})).unwrap();
        assert_eq!(serde_json::to_value(filter).unwrap()["side"], side);
    }
}

#[test]
fn prediction_history_requires_singular_token_id_and_prediction_interval() {
    let request = json!({"tokenId": "token_yes_123", "interval": "1H"});
    let params: GetPredictionPriceHistoryParams = serde_json::from_value(request.clone()).unwrap();
    assert_eq!(serde_json::to_value(params).unwrap(), request);
    assert!(
        serde_json::from_value::<GetPredictionPriceHistoryParams>(json!({"interval": "1H"}))
            .is_err()
    );
}

#[test]
fn prediction_history_decodes_object_result() {
    let result = json!({
        "tokenId": "token_yes_123", "interval": "1H",
        "list": [{"openTime": 1704067200000_i64, "open": "0.63",
                  "high": "0.68", "low": "0.62", "close": "0.65", "volume": "12500"}]
    });
    let response: GetPredictionPriceHistoryResponse =
        serde_json::from_value(result.clone()).unwrap();
    assert_eq!(serde_json::to_value(response).unwrap(), result);
}

#[test]
fn alpha_token_limits_decode_integer_and_fractional_numbers() {
    for minimum in [json!(1), json!(0.5)] {
        let result = json!([{
            "tokenCode": "token-1", "minOrderQuantity": minimum, "maxOrderQuantity": 50000
        }]);
        let response: GetBizTokenListResponse = serde_json::from_value(result).unwrap();
        let value = serde_json::to_value(response).unwrap();
        assert_eq!(value[0]["minOrderQuantity"], minimum);
        assert_eq!(value[0]["maxOrderQuantity"], 50000);
    }
}

#[test]
fn exchange_order_history_decodes_string_timestamps() {
    let response: QueryOrderByPageResponse = serde_json::from_value(json!({
        "orderBody": [{"fromCoin": "BTC", "createdTime": "1672197760"}],
        "nextPageCursor": ""
    }))
    .unwrap();
    assert_eq!(
        serde_json::to_value(response).unwrap()["orderBody"][0]["createdTime"],
        "1672197760"
    );
}

#[test]
fn prediction_side_markets_require_event_id_and_preserve_filters() {
    let request = json!({
        "eventId": "event_123", "sortBy": "volume", "marketType": 1,
        "limit": 20, "pageIndex": 1
    });
    let params: GetPredictionSideMarketListParams =
        serde_json::from_value(request.clone()).unwrap();
    assert_eq!(serde_json::to_value(params).unwrap(), request);
    assert!(
        serde_json::from_value::<GetPredictionSideMarketListParams>(json!({"eventType": 1}))
            .is_err()
    );
}

#[test]
fn prediction_group_details_accept_event_type_with_optional_group_name() {
    for request in [
        json!({"eventType": 1}),
        json!({"eventType": 1, "groupName": "A"}),
    ] {
        let params: GetPredictionGroupStageDetailParams =
            serde_json::from_value(request.clone()).unwrap();
        assert_eq!(serde_json::to_value(params).unwrap(), request);
    }
}
