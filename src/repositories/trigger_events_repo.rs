use crate::models::trigger_events::TriggerEvent;
use crate::schema::trigger_events_schema::NewTriggerEvent;
use sqlx::{Row, SqlitePool};

#[derive(Clone)]
pub struct TriggerEventRepo {
    db: SqlitePool,
}

impl TriggerEventRepo {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn insert_event(&self, event: &NewTriggerEvent) -> Result<TriggerEvent, sqlx::Error> {
        let entry = sqlx::query_as::<_, TriggerEvent>(
            r#"
            INSERT INTO trigger_events (trigger_type, wallet, value, token_mint, timestamp, tx_signature)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, trigger_type, wallet, value, token_mint, timestamp, tx_signature
            "#
        )
        .bind(event.trigger_type.match_type())
        .bind(&event.wallet)
        .bind(&event.value.to_string())
        .bind(&event.token_mint)
        .bind(&event.timestamp)
        .bind(&event.tx_signature)
        .fetch_one(&self.db)
        .await?;

        Ok(entry)
    }
}
