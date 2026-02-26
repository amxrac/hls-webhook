use crate::repositories::trigger_events_repo::TriggerEventRepo;
use axum::extract::FromRef;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: SqlitePool,
    pub trigger_events_repo: TriggerEventRepo,
}

impl AppState {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db = SqlitePool::connect(db_url).await?;

        sqlx::migrate!("./migrations").run(&db).await?;

        let trigger_events_repo = TriggerEventRepo::new(db.clone());

        Ok(Self {
            db,
            trigger_events_repo,
        })
    }
}
