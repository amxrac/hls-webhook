use std::env;

use axum::{
    Router,
    routing::{get, patch, post},
};
use hlswbhk::{handlers::*, state::*};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let app_state = AppState::new(&db_url).await.expect("db connection failed");

    println!("db connection successful!");

    let app = Router::new()
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
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server is running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
