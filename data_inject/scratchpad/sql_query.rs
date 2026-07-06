//Create table
CREATE TABLE Bitcoin_Data (
    agg_trade_id BIGINT PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL,
    event_time BIGINT NOT NULL,              -- Unix timestamp in milliseconds
    symbol VARCHAR(20) NOT NULL,
    price NUMERIC(20, 8) NOT NULL,           -- Handles high-precision crypto pricing
    quantity NUMERIC(20, 8) NOT NULL,        -- Handles high-precision crypto quantities
    is_buyer_market_maker BOOLEAN NOT NULL,  -- true = Market Sell, false = Market Buy
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
//create index
CREATE INDEX idx_agg_trades_symbol_time ON Bitcoin_Data (symbol, event_time DESC);
//Create table
CREATE TABLE Solana_Data (
    agg_trade_id BIGINT PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    bid_price NUMERIC(20, 8) NOT NULL,      -- High-precision best bid price
    bid_quantity NUMERIC(20, 8) NOT NULL,   -- High-precision volume at best bid
    ask_price NUMERIC(20, 8) NOT NULL,      -- High-precision best ask price
    ask_quantity NUMERIC(20, 8) NOT NULL,   -- High-precision volume at best ask
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
//create index
-- Index designed to optimize pulling the most recent order book state per symbol
CREATE INDEX idx_solana_symbol_created ON Solana_Data (symbol, created_at DESC);
