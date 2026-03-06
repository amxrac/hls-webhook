use crate::state::AppState;
use crate::{models::trigger_event::TriggerType, schema::workflow_schema::NewWorkflow};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};

pub async fn create_workflow(
    State(state): State<AppState>,
    Json(payload): Json<NewWorkflow>,
) -> (StatusCode, Json<Value>) {
    match state.workflow_repo.create_workflow(&payload).await {
        Ok(workflow) => (
            StatusCode::CREATED,
            Json(json!({
                "message": "workflow created successfully",
                 "workflow": workflow
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn get_all_workflows(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match state.workflow_repo.get_all_workflows().await {
        Ok(workflows) => (StatusCode::OK, Json(json!(workflows))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn get_active_workflows(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    match state.workflow_repo.get_active_workflows().await {
        Ok(workflows) => (StatusCode::OK, Json(json!(workflows))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn get_workflows_by_trigger_type(
    State(state): State<AppState>,
    Path(trigger_type): Path<TriggerType>,
) -> (StatusCode, Json<Value>) {
    match state
        .workflow_repo
        .get_workflows_by_trigger_type(trigger_type)
        .await
    {
        Ok(workflows) => (StatusCode::OK, Json(json!(workflows))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn pause_workflow(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> (StatusCode, Json<Value>) {
    match state.workflow_repo.update_status(id, "paused").await {
        Ok(Some(workflow)) => (StatusCode::OK, Json(json!(workflow))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "invalid id. workflow not found"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}

pub async fn activate_workflow(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> (StatusCode, Json<Value>) {
    match state.workflow_repo.update_status(id, "active").await {
        Ok(Some(workflow)) => (StatusCode::OK, Json(json!(workflow))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "invalid id. workflow not found"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        ),
    }
}
