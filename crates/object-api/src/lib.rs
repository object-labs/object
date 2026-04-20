use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub symbol: String,
    pub market_type: String,
    pub base_asset: String,
    pub quote_asset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInput {
    pub market: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub size: f64,
    pub price: Option<f64>,
    pub reduce_only: bool,
    pub account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub status: String,
    pub market: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub size: f64,
    pub price: Option<f64>,
    pub reduce_only: bool,
    pub account: String,
    pub created_at_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLevel {
    pub price: f64,
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub market: String,
    pub bids: Vec<OrderLevel>,
    pub asks: Vec<OrderLevel>,
    pub updated_at_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerpetualPosition {
    pub market: String,
    pub account: String,
    pub size: f64,
    pub entry_price: f64,
    pub unrealized_pnl: f64,
    pub margin_used: f64,
    pub mark_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingState {
    pub market: String,
    pub current_funding_rate: f64,
    pub next_funding_interval_ms: u64,
}
