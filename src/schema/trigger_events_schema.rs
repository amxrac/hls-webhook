use crate::models::trigger_events::TriggerType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTriggerEvent {
    pub trigger_type: TriggerType,
    pub wallet: String,
    pub value: f64,
    pub token_mint: Option<String>,
    pub timestamp: String,
    pub tx_signature: String,
}
