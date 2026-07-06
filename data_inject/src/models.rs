// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitcoinData {
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SolanaData {
    #[serde(rename = "u")]
    pub agg_trade_id: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "b")]
    pub bid_price: String,

    #[serde(rename = "B")]
    pub bid_quantity: String,
    
     #[serde(rename = "a")]
    pub ask_price: String,

    #[serde(rename = "A")]
    pub ask_quantity: String,
}