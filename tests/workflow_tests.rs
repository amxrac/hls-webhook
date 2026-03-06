use axum::http::StatusCode;
use serde_json::json;
mod common;

fn new_workflow(name: &str) -> serde_json::Value {
    json!({
        "name": name,
        "trigger_type": "swap",
        "condition_operator": "gt",
        "condition_value": 100.0,
        "watched_wallet": null,
        "watched_token_mint": null,
        "action_type": "NOTIFY",
        "action_params": "{}",
        "status": "active"
    })
}

#[tokio::test]
async fn test_create_workflow() {
    let res = common::server()
        .await
        .post("/workflows")
        .json(&new_workflow("test-workflow"))
        .await;
    res.assert_status(StatusCode::CREATED);
    let body = res.json::<serde_json::Value>();
    assert_eq!(body["message"], "workflow created successfully");
    assert_eq!(body["workflow"]["name"], "test-workflow");
    assert_eq!(body["workflow"]["status"], "active");
}

#[tokio::test]
async fn test_get_all_workflows() {
    let server = common::server().await;
    server.post("/workflows").json(&new_workflow("wf1")).await;
    server.post("/workflows").json(&new_workflow("wf2")).await;

    let res = server.get("/workflows").await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_get_active_workflows() {
    let server = common::server().await;
    server
        .post("/workflows")
        .json(&new_workflow("active-wf"))
        .await;

    let res = server.get("/workflows/active").await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    assert!(
        body.as_array()
            .unwrap()
            .iter()
            .all(|w| w["status"] == "active")
    );
}

#[tokio::test]
async fn test_pause_workflow() {
    let server = common::server().await;
    let created = server
        .post("/workflows")
        .json(&new_workflow("pause-wf"))
        .await
        .json::<serde_json::Value>();
    let id = created["workflow"]["id"].as_i64().unwrap();

    let res = server.post(&format!("/workflows/{id}/pause")).await;
    res.assert_status_ok();
    assert_eq!(res.json::<serde_json::Value>()["status"], "paused");
}

#[tokio::test]
async fn test_activate_workflow() {
    let server = common::server().await;
    let created = server
        .post("/workflows")
        .json(&new_workflow("activate-wf"))
        .await
        .json::<serde_json::Value>();
    let id = created["workflow"]["id"].as_i64().unwrap();

    server.post(&format!("/workflows/{id}/pause")).await;
    let res = server.post(&format!("/workflows/{id}/activate")).await;
    res.assert_status_ok();
    assert_eq!(res.json::<serde_json::Value>()["status"], "active");
}

#[tokio::test]
async fn test_pause_nonexistent_workflow() {
    let res = common::server().await.post("/workflows/99999/pause").await;
    res.assert_status(StatusCode::NOT_FOUND);
    assert_eq!(
        res.json::<serde_json::Value>()["error"],
        "invalid id. workflow not found"
    );
}

#[tokio::test]
async fn test_paused_workflow_excluded_from_active() {
    let server = common::server().await;
    let created = server
        .post("/workflows")
        .json(&new_workflow("to-pause"))
        .await
        .json::<serde_json::Value>();
    let id = created["workflow"]["id"].as_i64().unwrap();

    server.post(&format!("/workflows/{id}/pause")).await;

    let active = server
        .get("/workflows/active")
        .await
        .json::<serde_json::Value>();
    assert!(
        active
            .as_array()
            .unwrap()
            .iter()
            .all(|w| w["status"] != "paused")
    );
}

#[tokio::test]
async fn test_get_workflows_by_trigger_type() {
    let server = common::server().await;
    server
        .post("/workflows")
        .json(&new_workflow("swap-wf"))
        .await;

    let res = server.get("/workflows/type/swap").await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    assert!(body.is_array());
    assert!(
        body.as_array()
            .unwrap()
            .iter()
            .all(|w| w["trigger_type"] == "swap")
    );
}
