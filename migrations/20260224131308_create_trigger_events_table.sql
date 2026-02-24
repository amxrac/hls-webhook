-- Add migration script here
CREATE TABLE trigger_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    trigger_type TEXT NOT NULL,
    wallet TEXT NOT NULL,
    value TEXT NOT NULL,
    token_mint TEXT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tx_signature TEXT UNIQUE NOT NULL
);

CREATE INDEX idx_wallet ON trigger_events(wallet);
CREATE INDEX idx_token_mint ON trigger_events(token_mint);
