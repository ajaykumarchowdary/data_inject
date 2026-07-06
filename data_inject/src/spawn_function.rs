// src/spawn_function.rs
use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use crate::database::{Database, BulkInsertable};

pub fn spawn_worker<T>(url: String, token_label: &'static str, db_url: String)
where
    T: for<'a> serde::Deserialize<'a> + std::fmt::Debug + BulkInsertable + Send + 'static,
{
    tokio::spawn(async move {
        println!("[{}] Launching stream listener...", token_label);
        
        // 1. Initialize your Database connection client once using the passed string
        // (Assuming you add a basic initialization function like Database::new() in database.rs)
        let db = Database::new(&db_url).await.expect("Failed to initialize database client");
        
        let mut batch_buffer: Vec<T> = Vec::with_capacity(50);

        loop {
            match connect_async(&url).await {
                Ok((mut ws_stream, _)) => {
                    println!("[{}] Connected successfully! Streaming data below:", token_label);

                    while let Some(Ok(message)) = ws_stream.next().await {
                        if let Ok(text) = message.into_text() {
                            match serde_json::from_str::<T>(&text) {
                                Ok(data) => {
                                    println!("[{}] {:?}", token_label, data);
                                    batch_buffer.push(data);
                                    
                                    if batch_buffer.len() == 50 {
                                        // Pass the initialized db instance reference
                                        if let Err(e) = T::insert_bulk(&db, &batch_buffer).await {
                                            eprintln!("[{}] Database bulk insert failed: {:?}", token_label, e);
                                        }
                                        batch_buffer.clear(); 
                                    }
                                }
                                Err(e) => {
                                    eprintln!("[{}] Parsing Error: {} | Raw JSON was: {}", token_label, e, text);
                                }
                            }
                        }
                    }
                    eprintln!("[{}] Stream disconnected. Reconnecting in 5 seconds...", token_label);
                }
                Err(e) => {
                    eprintln!("[{}] Connection failed: {}. Retrying in 5 seconds...", token_label, e);
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });
}