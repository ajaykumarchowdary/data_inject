//use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
//use futures_util::StreamExt;
use tokio::io::{self, AsyncWriteExt};
//use std::env;
//Below to call other code files and variables
mod models;
//mod Spawn_worker;                 
use models::BitcoinData;
use models::SolanaData;
mod env_var;                
use env_var::{WEBSOCKET_URL, WEBSOCKET_PORT,BITCOIN_WEBSOCKET_PATH,DATABASE_URL,SOLANA_WEBSOCKET_PATH,BITCOIN_TOKEN_LABEL,SOLANA_TOKEN_LABEL};
// 1. Declare and import the database module
mod database;
use database::Database;
//Call function file to process it
mod spawn_function;
// Forces aggressive memory release after buffers are cleared
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = rustls::crypto::ring::default_provider().install_default();
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
    //stdout.write_all(b"Database pool established!\n").await?;
    // Allocate space for exactly 50 trade frames to prevent re-allocations
    //let url_string = format!("wss://{}:{}{}", WEBSOCKET_URL, WEBSOCKET_PORT, BITCOIN_WEBSOCKET_PATH);
    //println!("DEBUG: Connecting to hardcoded URL -> [{}]", url_string);
    // 2. Launch the concurrent worker pipelines
    spawn_function::spawn_worker::<BitcoinData>(format!("wss://{}:{}{}", WEBSOCKET_URL, WEBSOCKET_PORT, BITCOIN_WEBSOCKET_PATH).to_string(), BITCOIN_TOKEN_LABEL,DATABASE_URL.to_string());
    spawn_function::spawn_worker::<SolanaData>(format!("wss://{}:{}{}", WEBSOCKET_URL, WEBSOCKET_PORT, SOLANA_WEBSOCKET_PATH).to_string(), SOLANA_TOKEN_LABEL,DATABASE_URL.to_string());

    // 3. Keep main thread active so spawned tasks continue printing to the screen
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}