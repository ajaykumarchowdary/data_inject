use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use tokio::io::{self, AsyncWriteExt};
//Below to call other code files and variables
mod models;                 
use models::BitcoinData; 
mod env_var;                
use env_var::{WEBSOCKET_URL, WEBSOCKET_PORT,BITCOIN_WEBSOCKET_PATH,DATABASE_URL,SOLANA_WEBSOCKET_PATH};
// 1. Declare and import the database module
mod database;
use database::Database;
// Forces aggressive memory release after buffers are cleared
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let mut stdout = io::stdout();
    let mut trade_counter: u64 = 0;
    // 2. Initialize your database connection pool
    // Replace with your actual Postgres environment variable or fallback string
    println!("Step 1=================>>");
    let db_url = DATABASE_URL.to_string();
    //.unwrap_or_else(|_| "sqlite:///C:/Users/KuchipudiAjayKumar/Desktop/RUST/Projects/Database/test.db".to_string());
    println!("Step 2=================>>");
    println!("Databse connection [{}]", db_url);
    println!("DEBUG: Attempting to connect to database URL -> [{}]", db_url);
    let _db = Database::new(&db_url).await?;
    //let _db = Database::new(&db_url).await?;
    println!("Step 3=================>>");
    stdout.write_all(b"Database pool established!\n").await?;
    // Allocate space for exactly 50 trade frames to prevent re-allocations
    let mut batch_buffer: Vec<BitcoinData> = Vec::with_capacity(50);

    //let host = BITCOIN_WEBSOCKET_URL;   
    //let port = BITCOIN_WEBSOCKET_PORT;  
    //let ws_path = BITCOIN_WEBSOCKET_PATH;
    //let url_string = format!("wss://{}:{}{}", host, port, ws_path);
    //let url_string = format!("wss://{}:{}{}", WEBSOCKET_URL, WEBSOCKET_PORT, BITCOIN_WEBSOCKET_PATH);
    let urls = vec![
        format!("wss://{}:{}{}", WEBSOCKET_URL, WEBSOCKET_PORT, BITCOIN_WEBSOCKET_PATH).to_string(),
        format!("wss://{}:{}{}", WEBSOCKET_URL, WEBSOCKET_PORT, SOLANA_WEBSOCKET_PATH).to_string(),
        // Add as many URLs as you need here
    ];
    //println!("DEBUG: Connecting to hardcoded URL -> [{}]", url_string);

    let (mut ws_stream, _) = connect_async(&url_string).await?;
    stdout.write_all(b"WebSocket Connected successfully! Ingesting ticks...\n").await?;

    while let Some(msg) = ws_stream.next().await {
        if let Ok(Message::Text(text)) = msg {
            if let Ok(trade) = serde_json::from_slice::<BitcoinData>(text.as_bytes()) {
                
                // Add the parsed trade into our block storage
                batch_buffer.push(trade);

                // Once we hit exactly 50 records, process, print, and purge
                if batch_buffer.len() == 50 {
                    // ========================================================
                    // >>> ADD THIS BULK INSERT CALL HERE TO FIX THE ISSUE <<<
                    // ========================================================
                    if let Err(e) = _db.insert_bulk_trades(&batch_buffer).await {
                        eprintln!("Database bulk insert failed: {}", e);
                    }
                    let mut output_chunk = String::with_capacity(4096);
                    // 1. Change this line (remove the underscore)
                    //let _db = Database::new(&db_url).await?; // Was _db before
                    
                    for t in batch_buffer.iter() {
                        trade_counter += 1;
                        // Format the tick data into a compact, easily readable string row
                        let row = format!(
                            "Row: {:<6} | ID: {} | Price: {} | Qty: {} | Side: {}\n",
                            trade_counter,
                            t.agg_trade_id,
                            t.price,
                            t.quantity,
                            if t.is_buyer_market_maker { "SELL" } else { "BUY" }
                        );
                        output_chunk.push_str(&row);
                    }

                    // 1. Asynchronously dump all 50 text lines directly to the terminal screen
                    stdout.write_all(output_chunk.as_bytes()).await?;
                    stdout.flush().await?;

                    // 2. PURGE MEMORY: Clear the data elements out of the vector storage array.
                    // This forces the drop code to execute for all 50 trade models immediately.
                    batch_buffer.clear(); 
                }
            }
        }
    }
    Ok(())
}

                                    if token_label == BITCOIN_TOKEN_LABEL{
                                        Bitcoin_buffer.push(data);
                                    }
                                        if Bitcoin_buffer.len()==50 {
                                            if let Err(e) = _db.insert_bulk_trades(&batch_buffer).await {
                                            eprintln!("Database bulk insert failed: {}", e);
                                        }
                                        Bitcoin_buffer.clear(); 
                                    //println!("[{}] {:?}", token_label, data);

// src/database.rs
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, QueryBuilder};
use crate::models::BitcoinData;

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
    pub async fn insert_bulk_trades(&self, trades: &[BitcoinData]) -> Result<(), sqlx::Error> {
        if trades.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::new(
            "INSERT INTO Bitcoin_Data (agg_trade_id, event_type, event_time, symbol, price, quantity, is_buyer_market_maker) "
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