use crate::models::trigger_events::TriggerType;
use chrono::{DateTime, Utc};

pub struct NewTriggerEvent {
    pub trigger_type: TriggerType,
    pub wallet: String,
    pub value: f64,
    pub token_mint: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub tx_signature: String,
}
