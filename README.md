# Real-Time WebSocket Data Ingestion Engine

A ultra-high-performance, asynchronous data ingestion pipeline built in Rust. This project evaluates and benchmarks the performance capabilities of SQLite when handling dense, real-time financial market data streams using an asynchronous architecture.

## 🚀 Key Features
* **Zero-Block Async Pipeline:** Driven by `tokio` channels to decouple fast I/O ingestion from disk writes.
* **Polymorphic Trait Dispatch:** Employs a `StreamIngest` layout utilizing `Self` mapping to process independent data structures without messy conditional (`if/else`) branching.
* **Custom Memory Allocation (`mimalloc`):** Drops the standard OS system allocator in favor of Microsoft's `mimalloc`, drastically reducing lock contention and fragmentation across multithreaded Tokio workers.
* **SQLite Hyper-Optimization:** Uses Write-Ahead Logging (`WAL`), custom page caches, and transactional micro-batching to maximize database throughput.

---

## 🏗️ Architecture & Process Flow

The engine separates concern into three distinct computational layers to maximize processing speeds without freezing the event loop:

1. **Stream Layer (Network Source):** Receives raw WebSocket frames and deserializes them into model-specific structs.
2. **Buffer Layer (Tokio Channel):** Safely transports payloads over an asynchronous boundary.
3. **Storage Engine (SQLite Actor):** Accumulates data records up to a set batch limit or timeout threshold, then commits the entire batch in a single atomic database transaction.
