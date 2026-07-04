use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use tokio::io::{self, AsyncWriteExt};

mod models;                 
use models::AggTradeMessage; 
mod env_var;                
use env_var::{BINANCE_WEBSOCKET_URL, BINANCE_WEBSOCKET_PORT};
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
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:///C:/Users/KuchipudiAjayKumar/Desktop/RUST/Projects/data_ingection/Database/test.db".to_string());
    println!("Step 2=================>>");
    println!("Databse connection [{}]", db_url);
    println!("DEBUG: Attempting to connect to database URL -> [{}]", db_url);
    let _db = Database::new(&db_url).await?;
    //let _db = Database::new(&db_url).await?;
    println!("Step 3=================>>");
    stdout.write_all(b"Database pool established!\n").await?;
    // Allocate space for exactly 50 trade frames to prevent re-allocations
    let mut batch_buffer: Vec<AggTradeMessage> = Vec::with_capacity(50);

    //let host = BINANCE_WEBSOCKET_URL;   
    //let port = BINANCE_WEBSOCKET_PORT;  
    //let ws_path = "/ws/btcusdt@aggTrade";
    //let url_string = format!("wss://{}:{}{}", host, port, ws_path);
    // Hardcoded production URL bypasses env_var issues entirely
    //let url_string = "wss://stream.binance.com:9443/ws/btcusdt@aggTrade".to_string();
    let url_string = "wss://stream.binance.com/ws/btcusdt@aggTrade".to_string();
    println!("DEBUG: Connecting to hardcoded URL -> [{}]", url_string);

    let (mut ws_stream, _) = connect_async(&url_string).await?;
    stdout.write_all(b"WebSocket Connected successfully! Ingesting ticks...\n").await?;

    while let Some(msg) = ws_stream.next().await {
        if let Ok(Message::Text(text)) = msg {
            if let Ok(trade) = serde_json::from_slice::<AggTradeMessage>(text.as_bytes()) {
                
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