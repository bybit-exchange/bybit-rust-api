use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualAssetsProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_receive_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vip_product: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_end_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_quote_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_base_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_amount_quote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_amount_base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_precision_digital_quote: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_precision_digital_base: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdvanceProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DualAssetsProduct>>,
}

pub type GetAdvanceEarnProductResponse = GetAdvanceProductResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceQuote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy_e8: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_investment_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOffer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_low_price: Option<Vec<PriceQuote>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_high_price: Option<Vec<PriceQuote>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProductExtraInfoResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ProductOffer>>,
}

pub type GetAdvanceEarnProductExtraInfoResponse = GetProductExtraInfoResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualAssetsExtra {
    pub order_direction: String,
    pub select_price: String,
    pub apy_e8: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeverageStakeExtra {
    pub initial_price: String,
    pub breakeven_price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeverageRedeemExtra {
    pub position_id: String,
    pub est_redeem_amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_slippage_protected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceAdvanceEarnOrderParams {
    pub category: String,
    pub product_id: i64,
    pub order_type: String,
    pub amount: String,
    pub account_type: String,
    pub coin: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dual_assets_extra: Option<DualAssetsExtra>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_card: Option<InterestCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_leverage_stake_extra: Option<SmartLeverageStakeExtra>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_leverage_redeem_extra: Option<SmartLeverageRedeemExtra>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_win_stake_extra: Option<serde_json::Value>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_win_redeem_extra: Option<serde_json::Value>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_buy_extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceAdvanceOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

pub type PlaceAdvanceEarnOrderResponse = PlaceAdvanceOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualAssetsPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy_e8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_return_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_return_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_end_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdvancePositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DualAssetsPosition>>,
}

pub type GetAdvanceEarnPositionResponse = GetAdvancePositionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualAssetsOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_apy_e8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_apy_e8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_bonus_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_bonus_pnl: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdvanceOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DualAssetsOrder>>,
}

pub type GetAdvanceEarnOrderResponse = GetAdvanceOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeverageProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_receive_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_end_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_precision_digital: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeverageProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<SmartLeverageProduct>>,
}

pub type GetSmartLeverageProductResponse = GetSmartLeverageProductResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeverageProductExtraInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakeven_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_investment_amount: Option<String>,
}

pub type GetSmartLeverageProductExtraInfoResponse = SmartLeverageProductExtraInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeveragePosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakeven_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeveragePositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<SmartLeveragePosition>>,
}

pub type GetSmartLeveragePositionResponse = GetSmartLeveragePositionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartLeverageOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakeven_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSmartLeverageOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<SmartLeverageOrder>>,
}

pub type GetSmartLeverageOrderResponse = GetSmartLeverageOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRedeemEstAmountListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<serde_json::Value>>,
}

pub type GetRedeemEstAmountListResponse = GetRedeemEstAmountListResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinLeverageResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price: Option<String>,
}

pub type GetDoubleWinLeverageResponse = GetDoubleWinLeverageResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTokenOrderParams {
    pub coin: String,
    pub order_link_id: String,
    pub order_type: String,
    pub amount: String,
    pub account_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTokenOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

pub type PlaceTokenOrderResponse = PlaceTokenOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenOrderListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TokenOrder>>,
}

pub type TokenOrderListResponse = TokenOrderListResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_purchase_amount: Option<String>,
}

pub type TokenProductResponse = TokenProduct;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}

pub type TokenPositionResponse = TokenPosition;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDailyYield {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDailyYieldResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TokenDailyYield>>,
}

pub type TokenDailyYieldResponse = TokenDailyYieldResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenHourlyYield {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenHourlyYieldResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TokenHourlyYield>>,
}

pub type TokenHourlyYieldResponse = TokenHourlyYieldResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAprHistory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAprHistoryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<TokenAprHistory>>,
}

pub type TokenAprHistoryResponse = TokenAprHistoryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedTermProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedTermProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<FixedTermProduct>>,
}

pub type GetFixedTermProductResponse = GetFixedTermProductResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemEstItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub est_redeem_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub est_redeem_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_rate: Option<String>,
}

pub type GetSmartLeverageRedeemEstAmountListResponse = GetRedeemEstAmountListResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_receive_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_end_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_precision_digital: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rfq_product: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price_buffer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price_buffer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_deviation_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_deviation_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_tick_size: Option<String>,
}

pub type GetDoubleWinProductResponse = GetDoubleWinProductResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DoubleWinProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinProductExtraInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_investment_amount: Option<String>,
}

pub type GetDoubleWinProductExtraInfoResponse = DoubleWinProductExtraInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinStakeExtra {
    pub leverage: String,
    pub initial_price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinRedeemExtra {
    pub position_id: String,
    pub est_redeem_amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_slippage_protected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

pub type GetDoubleWinPositionResponse = GetDoubleWinPositionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinPositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DoubleWinPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleWinOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invest_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
}

pub type GetDoubleWinOrderResponse = GetDoubleWinOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDoubleWinOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<DoubleWinOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvDiscountBuyProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vip_product: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_end_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_purchase_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_precision_digital: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_receive_at: Option<i64>,
}

pub type GetDiscountBuyProductResponse = GetDiscountBuyProductResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountBuyProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AdvDiscountBuyProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountBuyPriceOfferItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_coupon_e8: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_investment_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_uid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountBuyProductExtraInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<DiscountBuyPriceOfferItem>>,
}

pub type GetDiscountBuyProductExtraInfoResponse = DiscountBuyProductExtraInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountBuyExtra {
    pub initial_price: String,
    pub purchase_price: String,
    pub knockout_price: String,
    pub knockout_coupon_e8: i64,
    pub inst_uid: i64,
    pub settle_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvDiscountBuyPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_coupon_e8: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_receive_at: Option<i64>,
}

pub type GetDiscountBuyPositionResponse = GetDiscountBuyPositionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountBuyPositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AdvDiscountBuyPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvDiscountBuyOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockout_coupon_e8: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<String>,
}

pub type GetDiscountBuyOrderResponse = GetDiscountBuyOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountBuyOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AdvDiscountBuyOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceFixedTermOrderParams {
    pub product_id: String,
    pub category: String,
    pub coin: String,
    pub amount: String,
    pub account_type: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_invest: Option<bool>,
}

pub type PlaceFixedTermOrderResponse = PlaceFixedTermOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceFixedTermOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemFixedTermParams {
    pub product_id: String,
    pub category: String,
    pub position_id: String,
}

pub type RedeemFixedTermResponse = RedeemFixedTermResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemFixedTermResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub est_redeem_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub est_redeem_time: Option<String>,
}

pub type GetFixedTermPositionResponse = GetFixedTermPositionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedTermPositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<FixedTermPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedTermPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_invest: Option<bool>,
}

pub type GetFixedTermOrderResponse = GetFixedTermOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedTermOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<FixedTermOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedTermOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFixedTermAutoInvestParams {
    pub product_id: String,
    pub category: String,
    pub position_id: String,
    pub status: String,
}

pub type RwaProductListResponse = RwaProductListResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaProductListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<RwaProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceRwaOrderParams {
    pub product_id: i64,
    pub order_type: String,
    pub coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub order_link_id: String,
}

pub type PlaceRwaOrderResponse = PlaceRwaOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceRwaOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

pub type RwaPositionListResponse = RwaPositionListResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaPositionListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<RwaPosition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

pub type RwaOrderListResponse = RwaOrderListResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaOrderListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<RwaOrder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

pub type RwaNavChartResponse = RwaNavChartResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaNavChartResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<RwaNavPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RwaNavPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
}

pub type GetProductResponse = GetProductResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProductResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<EarnProduct>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceEarnOrderParams {
    pub category: String,
    pub order_type: String,
    pub account_type: String,
    pub amount: String,
    pub coin: String,
    pub product_id: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_card: Option<serde_json::Value>,
}

pub type PlaceOrderResponse = PlaceOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

pub type OrderHistoryResponse = OrderHistoryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<EarnOrderHistoryItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnOrderHistoryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

pub type PositionResponse = PositionResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<EarnPositionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnPositionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

pub type YieldHistoryResponse = YieldHistoryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldHistoryResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<YieldHistoryItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldHistoryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyEarnPositionParams {
    pub category: String,
    pub product_id: i64,
    pub position_id: i64,
    pub auto_reinvest: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLiquidityParams {
    pub product_id: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveLiquidityParams {
    pub product_id: String,
    pub order_link_id: String,
    pub position_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReinvestLiquidityParams {
    pub product_id: String,
    pub order_link_id: String,
    pub position_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimLiquidityInterestParams {
    pub product_id: String,
}

pub type HourlyYieldResponse = HourlyYieldResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyYieldResult {
    #[serde(default)]
    pub list: Vec<HourlyYieldRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyYieldRecord {
    #[serde(default)]
    pub effective_amount: String,
    #[serde(default, rename = "yield")]
    pub yield_: String,
    #[serde(default)]
    pub reward_type: i32,
    #[serde(default)]
    pub apr_e8: i64,
    #[serde(default)]
    pub hourly_date: i64,
    #[serde(default)]
    pub created_time: i64,
}

pub type AprHistoryResponse = AprHistoryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AprHistoryResult {
    #[serde(default)]
    pub list: Vec<AprRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AprRecord {
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub apr_e8: i64,
}

pub type ListCouponsResponse = ListCouponsResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCouponsResult {
    #[serde(default)]
    pub list: Vec<EarnCoupon>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnCoupon {
    #[serde(default)]
    pub coupon_id: String,
    #[serde(default)]
    pub coupon_type: String,
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub end_time: i64,
}

pub type GetHoldToEarnProductResponse = GetHoldToEarnProductResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHoldToEarnProductResult {
    #[serde(default)]
    pub list: Vec<HoldToEarnProduct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldToEarnProduct {
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub min_stake_amount: String,
    #[serde(default)]
    pub max_stake_amount: String,
    #[serde(default)]
    pub precision: i32,
    #[serde(default)]
    pub apy: String,
}

pub type GetHoldToEarnYieldHistoryResponse = GetHoldToEarnYieldHistoryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHoldToEarnYieldHistoryResult {
    #[serde(default)]
    pub list: Vec<HoldToEarnYieldRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldToEarnYieldRecord {
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default, rename = "yield")]
    pub yield_: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_time: i64,
}

pub type GetLMProductsResponse = GetLMProductsResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMProductsResult {
    #[serde(default)]
    pub list: Vec<LMProduct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMProduct {
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub base_coin: String,
    #[serde(default)]
    pub quote_coin: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub min_leverage: String,
    #[serde(default)]
    pub max_leverage: String,
    #[serde(default)]
    pub min_quote_amount: String,
    #[serde(default)]
    pub max_quote_amount: String,
    #[serde(default)]
    pub min_base_amount: String,
    #[serde(default)]
    pub max_base_amount: String,
    #[serde(default)]
    pub apy: String,
    #[serde(default)]
    pub precision: i32,
}

pub type LMOrderResponse = LMOrderResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

pub type GetLMPositionsResponse = GetLMPositionsResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMPositionsResult {
    #[serde(default)]
    pub list: Vec<LMPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMPosition {
    #[serde(default)]
    pub position_id: String,
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub base_coin: String,
    #[serde(default)]
    pub quote_coin: String,
    #[serde(default)]
    pub base_amount: String,
    #[serde(default)]
    pub quote_amount: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub margin: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_time: i64,
}

pub type GetLMOrdersResponse = GetLMOrdersResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMOrdersResult {
    #[serde(default)]
    pub list: Vec<LMOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMOrder {
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub order_link_id: String,
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub position_id: String,
    #[serde(default)]
    pub order_type: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub base_coin: String,
    #[serde(default)]
    pub quote_coin: String,
    #[serde(default)]
    pub base_amount: String,
    #[serde(default)]
    pub quote_amount: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub created_time: i64,
    #[serde(default)]
    pub updated_time: i64,
}

pub type GetLMYieldRecordsResponse = GetLMYieldRecordsResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLMYieldRecordsResult {
    #[serde(default)]
    pub list: Vec<LMYieldRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMYieldRecord {
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub position_id: String,
    #[serde(default)]
    pub base_coin: String,
    #[serde(default)]
    pub quote_coin: String,
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_time: i64,
}

pub type GetTokenOrderListResponse = TokenOrderListResult;

pub type GetTokenProductResponse = TokenProduct;

pub type GetTokenPositionResponse = TokenPosition;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyYieldRecord {
    #[serde(default, rename = "yield")]
    pub yield_: String,
    #[serde(default)]
    pub bonus_yield: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_time: i64,
}

pub type GetTokenDailyYieldResponse = TokenDailyYieldResult;

pub type GetTokenHourlyYieldResponse = TokenHourlyYieldResult;

pub type GetTokenHistoricalAprResponse = TokenAprHistoryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TieredApy {
    #[serde(default)]
    pub min: String,
    #[serde(default)]
    pub max: String,
    #[serde(default)]
    pub apy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestCoinApy {
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub apy: String,
    #[serde(default)]
    pub expect_unit_earning: String,
    #[serde(default)]
    pub current_price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInterestCoinApy {
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub apy: String,
    #[serde(default)]
    pub expect_return_earning: String,
    #[serde(default)]
    pub price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldInfo {
    #[serde(default)]
    pub coin: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub apy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarlyRedeemInfo {
    #[serde(default)]
    pub allow_early_redeem: bool,
    #[serde(default)]
    pub early_redeem_earning: String,
    #[serde(default)]
    pub return_coin: String,
    #[serde(default)]
    pub redemption_limit_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstSettleProfitParams {
    pub fund_id: String,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstCreateFundParams {
    pub fund_name: String,
    pub coin: String,
    pub profit_share_rate: String,
    pub management_fee_rate: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_introduction: Option<String>,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstCreateInvestmentPlanParams {
    pub account_uid: String,
    pub plan_name: String,
    pub plan_type: String,
    pub investment_distribution: Vec<InvestmentDistributionItem>,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvestmentDistributionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstManageInvestmentPlanParams {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_funds: Option<Vec<InvestmentDistributionItem>>,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstManageOrderParams {
    pub order_id: String,
    pub action: String,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstCreateSubAccountParams {
    pub fund_id: String,
    pub req_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmFundTransferParams {
    pub transfer_id: String,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub amount: String,
    pub coin: String,
}

pub type GetLMLiquidationRecordsResponse = serde_json::Value;

pub type PwmInstListFundsResponse = serde_json::Value;

pub type PwmInstSettleProfitResponse = serde_json::Value;

pub type PwmInstCreateFundResponse = serde_json::Value;

pub type PwmInstCreateInvestmentPlanResponse = serde_json::Value;

pub type PwmInstGetInvestmentPlansResponse = serde_json::Value;

pub type PwmInstManageInvestmentPlanResponse = serde_json::Value;

pub type PwmInstListOrdersResponse = serde_json::Value;

pub type PwmInstManageOrderResponse = serde_json::Value;

pub type PwmInstCreateSubAccountResponse = serde_json::Value;

pub type PwmFundTransferResponse = serde_json::Value;

pub type PwmQueryFundTransferResponse = serde_json::Value;

pub type PwmListInvestmentPlansResponse = serde_json::Value;

pub type PwmGetPlanDetailResponse = serde_json::Value;

pub type PwmGetNewPlanDetailResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemFixedTermRequest {
    pub product_id: String,
    pub category: String,
    pub position_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoInvestRequest {
    pub product_id: String,
    pub category: String,
    pub position_id: String,
    pub status: String,
}

pub type GetRwaProductListResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceRwaOrderRequest {
    pub product_id: i64,
    pub order_type: String,
    pub coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub order_link_id: String,
}

pub type GetRwaPositionListResponse = serde_json::Value;

pub type GetRwaOrderListResponse = serde_json::Value;

pub type GetRwaNavChartResponse = serde_json::Value;

pub type GetEarnProductResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub category: String,
    pub order_type: String,
    pub account_type: String,
    pub amount: String,
    pub coin: String,
    pub product_id: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_card: Option<InterestCardRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestCardRef {
    pub award_id: i64,
    pub spec_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmClaimParams {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    pub order_link_id: String,
}

pub type PwmClaimResponse = serde_json::Value;

pub type PwmAssetTrendResponse = serde_json::Value;

pub type PwmFundNavResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmSubscribeParams {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub order_link_id: String,
}

pub type PwmSubscribeResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestMoreParams {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub category: String,
    pub product_id: String,
    pub amount: String,
    pub order_link_id: String,
}

pub type PwmInvestMoreResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmRedeemParams {
    pub plan_id: String,
    pub category: String,
    pub product_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<i64>,
}

pub type PwmRedeemResponse = serde_json::Value;

pub type PwmListOrderResponse = serde_json::Value;

pub type PwmListProductCardsResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCreateCustomPlanParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub products: Vec<serde_json::Value>,
    pub order_link_id: String,
}

pub type PwmCreateCustomPlanResponse = serde_json::Value;

pub type PlaceEarnOrderResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_order_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_redeem_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_stake_time: Option<String>,
}

pub type GetEarnOrderHistoryResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimable_yield: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_redeem_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_stake_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_interest_calculation_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_reinvest: Option<String>,
}

pub type GetEarnPositionResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_staking_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

pub type GetEarnYieldHistoryResponse = serde_json::Value;

pub type GetEarnHourlyYieldHistoryResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyEarnPositionRequest {
    pub category: String,
    pub product_id: i64,
    pub position_id: i64,
    pub auto_reinvest: i32,
}

pub type GetEarnAprHistoryResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwardCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_pnl_percentage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

pub type ListEarnCouponsResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldToEarnYieldCoin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirdropDailyPnl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_coin_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinApy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<i64>,
    #[serde(rename = "apy_e8", skip_serializing_if = "Option::is_none")]
    pub apy_e8: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<bool>,
}

pub type GetLiquidityMiningProductsResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLiquidityRequest {
    pub product_id: String,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveLiquidityRequest {
    pub product_id: String,
    pub order_link_id: String,
    pub position_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReinvestRequest {
    pub product_id: String,
    pub order_link_id: String,
    pub position_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarginRequest {
    pub product_id: String,
    pub order_link_id: String,
    pub position_id: String,
    pub amount: String,
    pub quote_account_type: String,
}

pub type AddLiquidityResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimInterestRequest {
    pub product_id: String,
}

pub type GetLiquidityMiningPositionsResponse = serde_json::Value;

pub type GetLiquidityMiningOrdersResponse = serde_json::Value;

pub type GetLiquidityMiningYieldRecordsResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LMLiquidationRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation_time: Option<String>,
}

pub type GetLiquidityMiningLiquidationRecordsResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstFundInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_equity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_nav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentAPR")]
    pub current_apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_share_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_fee_rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncollected_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collected_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_loan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstInvestmentDistributionItem {
    pub product_id: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstInvestmentDistribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstInvestmentPlanInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_asset_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investment_distribution: Option<Vec<PwmInstInvestmentDistribution>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

// `ClaimLiquidityInterestResponse` carries no typed result body — the endpoint
// returns only the envelope. After `client.rs::parse_response` unwraps the
// envelope, the inner `result` is whatever Bybit returns (often `{}`).
pub type ClaimLiquidityInterestResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCoinAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestmentDistribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmPositionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_reinvest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCategoryPositionGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_investment_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_avg_apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PwmPositionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmEquityFundPositionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holding_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr30d: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr_total: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharp_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_drawdown: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmEquityFundPositionGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_investment_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_avg_apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PwmEquityFundPositionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmPlanPositions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_coins_earning: Option<PwmCategoryPositionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_yield: Option<PwmCategoryPositionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equity_funds: Option<PwmEquityFundPositionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onchain_earn: Option<PwmCategoryPositionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_account: Option<Vec<PwmCoinAmount>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmFundIntroduction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_yield_rate_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_yield_rate_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharp_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_drawback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockup_period: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmConfiguredProductItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduction: Option<PwmFundIntroduction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmConfiguredProductGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_amount_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PwmConfiguredProductItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmNewPlanProducts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_coins_earning: Option<PwmConfiguredProductGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_yield: Option<PwmConfiguredProductGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equity_funds: Option<PwmConfiguredProductGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onchain_earn: Option<PwmConfiguredProductGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestmentPlanSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_asset_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investment_distribution: Option<Vec<PwmInvestmentDistribution>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstUpdateFundItem {
    pub fund_id: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInstOrderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmFundTransferRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

pub type PwmQueryFundTransferResultResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmPlanDetailResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_asset_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulate_yield_usd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_avg_apr: Option<String>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_assets: Option<serde_json::Value>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmNewPlanDetailResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_configured_amount_usd: Option<String>,
    // FIXME(typed-field): falls back to `serde_json::Value` because the Bybit
    // spec did not provide a matching inner type at generation time. Replace
    // with a typed struct in a follow-up PR after consulting the V5 docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmClaimResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmAssetDataPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_value_usd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmAssetTrendResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_points: Option<Vec<PwmAssetDataPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmNavDataPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmFundNavResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_nav: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_points: Option<Vec<PwmNavDataPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmSubscribeRequest {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmSubscribeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestMoreRequest {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub category: String,
    pub product_id: String,
    pub amount: String,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmInvestMoreResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmRedeemRequest {
    pub plan_id: String,
    pub category: String,
    pub product_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    pub order_link_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmRedeemResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmOrderDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmListOrderResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<PwmOrderDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmProductCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr_range_low: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apr_range_high: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_investment_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_investment_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_drawdown: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharp_ratio: Option<String>,
    #[serde(rename = "estAPR", skip_serializing_if = "Option::is_none")]
    pub est_apr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmProductTypeGroup {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<PwmProductCard>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmListProductCardsResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<PwmProductTypeGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCreateCustomPlanProductItem {
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_name: Option<String>,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCreateCustomPlanRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    pub products: Vec<PwmCreateCustomPlanProductItem>,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmCreateCustomPlanResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwmClaimRequest {
    pub plan_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
    pub order_link_id: String,
}

pub type ListEarnAutoSavingsResponse = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditEarnAutoSavingsParams {
    #[serde(default)]
    pub coin: Option<String>,
    pub is_selected: bool,
    #[serde(default)]
    pub purchase_immediately: Option<bool>,
}

pub type EditEarnAutoSavingsResponse = serde_json::Value;
