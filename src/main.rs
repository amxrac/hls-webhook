use std::env;

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    response::Response,
    routing::{get, post},
};
use hlswbhk::{
    parser::{self, parse_event},
    repositories::trigger_event_repo::TriggerEventRepo,
    state::AppState,
};
use serde_json::{Value, json};

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
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server is running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"status": "ok", "db": "connected"})),
        ),
        Err(e) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"status": "error", "error": e.to_string()})),
        ),
    }
}

async fn webhook(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let txs = match payload.as_array() {
        Some(arr) => arr,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "invalid format"})),
            );
        }
    };

    let mut saved_events = vec![];
    let mut skipped_events = 0;

    for tx in txs {
        if let Some(new_event) = parse_event(tx) {
            match state.trigger_events_repo.insert_event(&new_event).await {
                Ok(Some(event)) => saved_events.push(event),
                Ok(None) => skipped_events += 1,
                Err(e) => {
                    eprintln!("db insert error: {e}");
                    skipped_events += 1;
                }
            }
        } else {
            skipped_events += 1;
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "saved events": saved_events.len(),
            "skipped events": skipped_events,
            "events": saved_events,
        })),
    )
}

async fn get_all_events(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match state.trigger_events_repo.get_all_events().await {
        Ok(events) => (StatusCode::OK, Json(json!(events))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

async fn get_events_by_wallet(
    State(state): State<AppState>,
    Path(wallet): Path<String>,
) -> (StatusCode, Json<Value>) {
    match state
        .trigger_events_repo
        .get_events_by_wallet(&wallet)
        .await
    {
        Ok(events) => (StatusCode::OK, Json(json!(events))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

async fn get_events_by_token_mint(
    State(state): State<AppState>,
    Path(token_mint): Path<String>,
) -> (StatusCode, Json<Value>) {
    match state
        .trigger_events_repo
        .get_events_by_token_mint(&token_mint)
        .await
    {
        Ok(events) => (StatusCode::OK, Json(json!(events))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}
