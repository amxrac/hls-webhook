use std::env;

use axum::{Router, extract::State, http::StatusCode, response::Json, routing::get, response::Response};
use hlswbhk::state::AppState;
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let app_state = AppState::new(&db_url).await.expect("db connection failed");

    println!("db connection successful!");

    let app = Router::new()
        .route("/health", get(health_check))
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

async fn webhook(State(state): State<AppState>) ->
