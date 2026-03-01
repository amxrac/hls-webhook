use crate::models::trigger_event::TriggerType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWorkflow {
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
