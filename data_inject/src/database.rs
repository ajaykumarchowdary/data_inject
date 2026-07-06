// src/database.rs

use crate::models::{BitcoinData, SolanaData};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, QueryBuilder};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyError(pub String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database Error: {}", self.0)
    }
}
impl Error for MyError {}

pub struct Database {
    pub pool: SqlitePool, // Swapped to SqlitePool
}

impl Database {
    pub async fn new(db_url: &str) -> Result<Self, MyError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            .map_err(|e| MyError(format!("Failed to connect to SQLite: {}", e)))?;

        Ok(Self { pool })
    }

    pub async fn insert_bitcoin_bulk(&self, batch: &[BitcoinData]) -> Result<(), MyError> {
        if batch.is_empty() { return Ok(()); }

        // SQLite bulk inserts use dynamically constructed QueryBuilders
        let mut query_builder = QueryBuilder::new(
            "INSERT OR IGNORE INTO Bitcoin_Data (agg_trade_id, event_type, event_time, symbol, price, quantity, is_buyer_market_maker) "
        );

        // Map every row value into standard SQL parameters
        query_builder.push_values(batch, |mut b, trade| {
            b.push_bind(trade.agg_trade_id as i64)
             .push_bind(&trade.event_type)
             .push_bind(trade.event_time as i64)
             .push_bind(&trade.symbol)
             .push_bind(trade.price.parse::<f64>().unwrap_or(0.0))
             .push_bind(trade.quantity.parse::<f64>().unwrap_or(0.0))
             .push_bind(trade.is_buyer_market_maker);
        });

        let query = query_builder.build();
        query.execute(&self.pool)
            .await
            .map_err(|e| MyError(format!("Bitcoin SQLite bulk insert failed: {}", e)))?;

        println!("Successfully bulk inserted {} Bitcoin trades to SQLite.", batch.len());
        Ok(())
    }

    pub async fn insert_solana_bulk(&self, batch: &[SolanaData]) -> Result<(), MyError> {
        if batch.is_empty() { return Ok(()); }

        let mut query_builder = QueryBuilder::new(
            "INSERT OR IGNORE INTO Solana_Data (agg_trade_id, symbol, bid_price, bid_quantity, ask_price, ask_quantity) "
        );

        query_builder.push_values(batch, |mut b, order| {
            b.push_bind(order.agg_trade_id as i64)
             .push_bind(&order.symbol)
             .push_bind(order.bid_price.parse::<f64>().unwrap_or(0.0))
             .push_bind(order.bid_quantity.parse::<f64>().unwrap_or(0.0))
             .push_bind(order.ask_price.parse::<f64>().unwrap_or(0.0))
             .push_bind(order.ask_quantity.parse::<f64>().unwrap_or(0.0));
        });

        let query = query_builder.build();
        query.execute(&self.pool)
            .await
            .map_err(|e| MyError(format!("Solana SQLite bulk insert failed: {}", e)))?;

        println!("Successfully bulk inserted {} Solana updates to SQLite.", batch.len());
        Ok(())
    }
}

// 2. The Polymorphic Trait Blueprint using thread-safe Future syntax
pub trait BulkInsertable: Sized + Send + Sync {
    fn insert_bulk(db: &Database, batch: &[Self]) -> impl std::future::Future<Output = Result<(), MyError>> + Send;
}

// 3. Bind the trait behavior to BitcoinData
impl BulkInsertable for BitcoinData {
    fn insert_bulk(db: &Database, batch: &[Self]) -> impl std::future::Future<Output = Result<(), MyError>> + Send {
        async move {
            db.insert_bitcoin_bulk(batch).await
        }
    }
}

// 4. Bind the trait behavior to SolanaData
impl BulkInsertable for SolanaData {
    fn insert_bulk(db: &Database, batch: &[Self]) -> impl std::future::Future<Output = Result<(), MyError>> + Send {
        async move {
            db.insert_solana_bulk(batch).await
        }
    }
}