use axum_test::TestServer;
use hlswbhk::{router, state::AppState};
use sqlx::sqlite::SqlitePoolOptions;

pub async fn server() -> TestServer {
    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&db).await.unwrap();
    let test_app_state = AppState::from_pool(db);
    TestServer::new(router(test_app_state))
}
