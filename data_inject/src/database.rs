// src/database.rs
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, QueryBuilder};
use crate::models::AggTradeMessage;

pub struct Database {
    pool: SqlitePool, // Swapped from PgPool
}

impl Database {
    /// Initializes a new SQLite connection pool
   // src/database.rs inside impl Database
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
    use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
    use std::str::FromStr;

    // Build connection options manually to force instant disk synchronization
    let connection_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal) // Fast write-ahead logging
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal); // Fast but reliable commits

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options) // Use the explicit options
        .await?;
        
    Ok(Self { pool })
    }

    /// Performs an efficient bulk insert of up to 50 trade records into SQLite
    pub async fn insert_bulk_trades(&self, trades: &[AggTradeMessage]) -> Result<(), sqlx::Error> {
        if trades.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::new(
            "INSERT INTO agg_trades (agg_trade_id, event_type, event_time, symbol, price, quantity, is_buyer_market_maker) "
        );

        query_builder.push_values(trades, |mut b, trade| {
            // Note: SQLite safely stores text/strings for high-precision decimals natively!
            // We can pass your struct's String fields directly without BigDecimal conversion.
            b.push_bind(trade.agg_trade_id as i64)
             .push_bind(&trade.event_type)
             .push_bind(trade.event_time as i64)
             .push_bind(&trade.symbol)
             .push_bind(&trade.price)      // String bind
             .push_bind(&trade.quantity)   // String bind
             .push_bind(trade.is_buyer_market_maker);
        });

        // SQLite syntax for upsert conflict handling
        query_builder.push(" ON CONFLICT (agg_trade_id) DO NOTHING");

        let query = query_builder.build();
        query.execute(&self.pool).await?;

        Ok(())
    }
}