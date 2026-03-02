use crate::parser::parse_event;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};

pub async fn webhook(
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

pub async fn get_all_events(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match state.trigger_events_repo.get_all_events().await {
        Ok(events) => (StatusCode::OK, Json(json!(events))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn get_events_by_wallet(
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

pub async fn get_events_by_token_mint(
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
