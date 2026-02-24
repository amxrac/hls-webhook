use axum::extract::FromRef;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db = SqlitePool::connect(db_url).await?;

        Ok(Self { db })
    }
}
