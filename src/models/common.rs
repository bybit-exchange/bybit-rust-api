//! Common types used across the Bybit API.

use serde::{Deserialize, Serialize};

/// A string newtype that hides its contents from `Debug` formatting.
///
/// Use for API secrets, passwords, and other credentials that must never
/// appear in logs. Serializes/deserializes transparently as a plain string
/// so the wire format is unchanged.
///
/// Call [`Self::expose_secret`] to access the underlying value when you
/// genuinely need it (e.g. to authenticate a request).
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RedactedString(String);

impl RedactedString {
    /// Wrap a string. Prefer [`From<String>`] / [`From<&str>`] in code.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Access the underlying secret. Use sparingly; never log the return
    /// value of this method.
    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for RedactedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Note: do NOT include length or any prefix/suffix of the secret;
        // doing so can leak entropy in low-entropy secrets.
        f.write_str("RedactedString(***)")
    }
}

impl From<String> for RedactedString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for RedactedString {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

/// Forward-compatible scalar value for generated OpenAPI enums whose
/// authoritative variants are not available to the generator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenApiScalar {
    String(String),
    Signed(i64),
    Unsigned(u64),
    Boolean(bool),
}

impl From<String> for OpenApiScalar {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for OpenApiScalar {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<i64> for OpenApiScalar {
    fn from(value: i64) -> Self {
        Self::Signed(value)
    }
}

impl From<u64> for OpenApiScalar {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

impl From<bool> for OpenApiScalar {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

/// Product category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// Spot trading
    Spot,
    /// Linear perpetual (USDT margined)
    Linear,
    /// Inverse perpetual/futures
    Inverse,
    /// Options
    Option,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spot => write!(f, "spot"),
            Self::Linear => write!(f, "linear"),
            Self::Inverse => write!(f, "inverse"),
            Self::Option => write!(f, "option"),
        }
    }
}

/// Order side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "Buy"),
            Self::Sell => write!(f, "Sell"),
        }
    }
}

/// Order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Market order
    Market,
    /// Limit order
    Limit,
}

/// Time in force.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Good till cancelled
    GTC,
    /// Immediate or cancel
    IOC,
    /// Fill or kill
    FOK,
    /// Post only
    PostOnly,
}

/// Order status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// Order created
    Created,
    /// Order is new (not filled)
    New,
    /// Order is rejected
    Rejected,
    /// Order is partially filled
    PartiallyFilled,
    /// Order is partially filled and cancelled
    PartiallyFilledCanceled,
    /// Order is fully filled
    Filled,
    /// Order is cancelled
    Cancelled,
    /// Order is untriggered
    Untriggered,
    /// Order is triggered
    Triggered,
    /// Order is deactivated
    Deactivated,
    /// Order is active (conditional)
    Active,
}

/// Position side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionIdx {
    /// One-way mode
    #[serde(rename = "0")]
    OneWay = 0,
    /// Hedge mode - buy side
    #[serde(rename = "1")]
    HedgeBuy = 1,
    /// Hedge mode - sell side
    #[serde(rename = "2")]
    HedgeSell = 2,
}

/// Account type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    /// Contract account
    CONTRACT,
    /// Unified account
    UNIFIED,
    /// Spot account
    SPOT,
    /// Investment account
    INVESTMENT,
    /// Option account
    OPTION,
    /// Fund account
    FUND,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CONTRACT => write!(f, "CONTRACT"),
            Self::UNIFIED => write!(f, "UNIFIED"),
            Self::SPOT => write!(f, "SPOT"),
            Self::INVESTMENT => write!(f, "INVESTMENT"),
            Self::OPTION => write!(f, "OPTION"),
            Self::FUND => write!(f, "FUND"),
        }
    }
}

/// Kline interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interval {
    /// 1 minute
    #[serde(rename = "1")]
    M1,
    /// 3 minutes
    #[serde(rename = "3")]
    M3,
    /// 5 minutes
    #[serde(rename = "5")]
    M5,
    /// 15 minutes
    #[serde(rename = "15")]
    M15,
    /// 30 minutes
    #[serde(rename = "30")]
    M30,
    /// 1 hour
    #[serde(rename = "60")]
    H1,
    /// 2 hours
    #[serde(rename = "120")]
    H2,
    /// 4 hours
    #[serde(rename = "240")]
    H4,
    /// 6 hours
    #[serde(rename = "360")]
    H6,
    /// 12 hours
    #[serde(rename = "720")]
    H12,
    /// 1 day
    #[serde(rename = "D")]
    D1,
    /// 1 week
    #[serde(rename = "W")]
    W1,
    /// 1 month
    #[serde(rename = "M")]
    M1Month,
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::M1 => write!(f, "1"),
            Self::M3 => write!(f, "3"),
            Self::M5 => write!(f, "5"),
            Self::M15 => write!(f, "15"),
            Self::M30 => write!(f, "30"),
            Self::H1 => write!(f, "60"),
            Self::H2 => write!(f, "120"),
            Self::H4 => write!(f, "240"),
            Self::H6 => write!(f, "360"),
            Self::H12 => write!(f, "720"),
            Self::D1 => write!(f, "D"),
            Self::W1 => write!(f, "W"),
            Self::M1Month => write!(f, "M"),
        }
    }
}

/// Trigger price type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerBy {
    /// Last price
    LastPrice,
    /// Index price
    IndexPrice,
    /// Mark price
    MarkPrice,
}

/// Position mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionMode {
    /// Merged single position (one-way)
    #[serde(rename = "0")]
    MergedSingle = 0,
    /// Both sides (hedge mode)
    #[serde(rename = "3")]
    BothSides = 3,
}

/// Margin mode.
///
/// Wire protocol values follow Bybit V5 `/v5/account/set-margin-mode`:
/// - `ISOLATED_MARGIN`
/// - `REGULAR_MARGIN` (mapped from [`Self::CROSS`])
/// - `PORTFOLIO_MARGIN` (mapped from [`Self::PORTFOLIO`])
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarginMode {
    /// Cross margin (wire value: `REGULAR_MARGIN`)
    #[serde(rename = "REGULAR_MARGIN")]
    CROSS,
    /// Isolated margin (wire value: `ISOLATED_MARGIN`)
    #[serde(rename = "ISOLATED_MARGIN")]
    ISOLATED,
    /// Portfolio margin (wire value: `PORTFOLIO_MARGIN`)
    #[serde(rename = "PORTFOLIO_MARGIN")]
    PORTFOLIO,
}

impl std::fmt::Display for MarginMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CROSS => write!(f, "REGULAR_MARGIN"),
            Self::ISOLATED => write!(f, "ISOLATED_MARGIN"),
            Self::PORTFOLIO => write!(f, "PORTFOLIO_MARGIN"),
        }
    }
}

/// TP/SL mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TpSlMode {
    /// Full position TP/SL
    Full,
    /// Partial position TP/SL
    Partial,
}
