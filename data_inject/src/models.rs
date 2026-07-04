// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AggTradeMessage {
    #[serde(rename = "e")]
    pub event_type: String,
    
    #[serde(rename = "E")]
    pub event_time: u64,
    
    #[serde(rename = "s")]
    pub symbol: String,
    
    #[serde(rename = "a")]
    pub agg_trade_id: u64,
    
    #[serde(rename = "p")]
    pub price: String,
    
    #[serde(rename = "q")]
    pub quantity: String,
    
    #[serde(rename = "m")]
    pub is_buyer_market_maker: bool, // true = Market Sell, false = Market Buy
}