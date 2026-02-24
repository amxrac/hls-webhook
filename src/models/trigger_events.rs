use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TriggerEvent {
    pub id: i64,
    pub trigger_type: TriggerType,
    pub wallet: String,
    pub value: f64,
    pub token_mint: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub tx_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(rename_all = "snake_case")]
pub enum TriggerType {
    TokenTranfer,
    WalletBalance,
    Swap,
}
