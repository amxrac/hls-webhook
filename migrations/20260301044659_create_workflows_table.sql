-- Add migration script here
CREATE TABLE workflows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    trigger_type TEXT NOT NULL,
    condition_operator TEXT NOT NULL,
    condition_value REAL NOT NULL,
    watched_wallet TEXT,
    watched_token_mint TEXT,
    action_type TEXT NOT NULL,
    action_params TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_workflows_status ON workflows(status);
CREATE INDEX idx_workflows_trigger_type ON workflows(trigger_type);
