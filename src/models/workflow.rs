use crate::models::trigger_event::TriggerType;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Workflow {
    pub id: i64,
    pub name: String,
    pub trigger_type: TriggerType,
    pub condition_operator: String,
    pub condition_value: f64,
    pub watched_wallet: Option<String>,
    pub watched_token_mint: Option<String>,
    pub action_type: String,
    pub action_params: String,
    pub status: String,
    pub created_at: String,
}
