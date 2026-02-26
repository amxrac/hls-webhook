use crate::models::trigger_events::TriggerEvent;
use crate::schema::trigger_events_schema::NewTriggerEvent;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct TriggerEventRepo {
    db: SqlitePool,
}

impl TriggerEventRepo {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn insert_event(
        &self,
        event: &NewTriggerEvent,
    ) -> Result<Option<TriggerEvent>, sqlx::Error> {
        let entry = sqlx::query_as::<_, TriggerEvent>(
            r#"
            INSERT OR IGNORE INTO trigger_events (trigger_type, wallet, value, token_mint, timestamp, tx_signature)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id, trigger_type, wallet, value, token_mint, timestamp, tx_signature
            "#
        )
        .bind(event.trigger_type.match_type())
        .bind(&event.wallet)
        .bind(&event.value)
        .bind(&event.token_mint)
        .bind(&event.timestamp)
        .bind(&event.tx_signature)
        .fetch_optional(&self.db)
        .await?;

        Ok(entry)
    }

    pub async fn get_all_events(&self) -> Result<Vec<TriggerEvent>, sqlx::Error> {
        let events = sqlx::query_as::<_, TriggerEvent>(
            r#"
            SELECT id, trigger_type, wallet, value, token_mint, timestamp, tx_signature
            FROM trigger_events
            LIMIT 30
            "#,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(events)
    }

    pub async fn get_events_by_wallet(
        &self,
        wallet: &str,
    ) -> Result<Vec<TriggerEvent>, sqlx::Error> {
        let event = sqlx::query_as::<_, TriggerEvent>(
            r#"
            SELECT id, trigger_type, wallet, value, token_mint, timestamp, tx_signature
            FROM trigger_events
            WHERE wallet = ?
            ORDER BY timestamp DESC
            LIMIT 30
            "#,
        )
        .bind(wallet)
        .fetch_all(&self.db)
        .await?;

        Ok(event)
    }

    pub async fn get_events_by_token_mint(
        &self,
        token_mint: &str,
    ) -> Result<Vec<TriggerEvent>, sqlx::Error> {
        let event = sqlx::query_as::<_, TriggerEvent>(
            r#"
            SELECT id, trigger_type, wallet, value, token_mint, timestamp, tx_signature
            FROM trigger_events
            WHERE token_mint = ?
            ORDER BY timestamp DESC
            LIMIT 30
            "#,
        )
        .bind(token_mint)
        .fetch_all(&self.db)
        .await?;

        Ok(event)
    }
}
