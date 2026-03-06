pub mod handlers;
pub mod models;
pub mod parser;
pub mod repositories;
pub mod schema;
pub mod state;
use std::env;

use crate::{handlers::*, state::*};
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/webhook", post(webhook))
        .route("/events", get(get_all_events))
        .route("/events/{wallet}", get(get_events_by_wallet))
        .route("/events/mint/{token_mint}", get(get_events_by_token_mint))
        .route("/workflows", post(create_workflow))
        .route("/workflows", get(get_all_workflows))
        .route("/workflows/active", get(get_active_workflows))
        .route("/workflows/{id}/pause", post(pause_workflow))
        .route("/workflows/{id}/activate", post(activate_workflow))
        .route(
            "/workflows/type/{trigger_type}",
            get(get_workflows_by_trigger_type),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

pub async fn app() -> Router {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let app_state = AppState::new(&db_url).await.expect("db connection failed");

    println!("db connection successful!");

    router(app_state)
}
