use crate::models::trigger_event::TriggerType;
use crate::models::workflow::Workflow;
use crate::schema::workflow_schema::NewWorkflow;

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct WorkflowRepo {
    db: SqlitePool,
}

impl WorkflowRepo {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn create_workflow(
        &self,
        workflow: &NewWorkflow,
    ) -> Result<Option<Workflow>, sqlx::Error> {
        let workflow = sqlx::query_as::<_, Workflow>(
            r#"
            INSERT INTO workflows (name, trigger_type, condition_operator, condition_value, watched_wallet, watched_token_mint, action_type, action_params, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id, name, trigger_type, condition_operator, condition_value, watched_wallet, watched_token_mint, action_type, action_params, status, created_at
            "#
        )
        .bind(&workflow.name)
        .bind(workflow.trigger_type.match_type())
        .bind(&workflow.condition_operator)
        .bind(&workflow.condition_value)
        .bind(&workflow.watched_wallet)
        .bind(&workflow.watched_token_mint)
        .bind(&workflow.action_type)
        .bind(&workflow.action_params)
        .bind(&workflow.status)
        .bind(&workflow.created_at)
        .fetch_optional(&self.db)
        .await?;

        Ok(workflow)
    }

    pub async fn get_active_workflows(&self) -> Result<Vec<Workflow>, sqlx::Error> {
        let workflows = sqlx::query_as::<_, Workflow>(
            r#"
            SELECT id, name, trigger_type, condition_operator, condition_value, watched_wallet, watched_token_mint, action_type, action_params, status, created_at
            FROM workflows
            WHERE status = 'active'
            ORDER BY created_at DESC
            LIMIT 30
            "#
        )
        .fetch_all(&self.db)
        .await?;

        Ok(workflows)
    }

    pub async fn get_all_workflows(&self) -> Result<Vec<Workflow>, sqlx::Error> {
        let workflows = sqlx::query_as::<_, Workflow>(
            r#"
            SELECT id, name, trigger_type, condition_operator, condition_value, watched_wallet, watched_token_mint, action_type, action_params, status, created_at
            FROM workflows
            ORDER BY created_at DESC
            LIMIT 30
            "#
        )
        .fetch_all(&self.db)
        .await?;

        Ok(workflows)
    }

    pub async fn get_workflows_by_trigger_type(
        &self,
        trigger_type: TriggerType,
    ) -> Result<Vec<Workflow>, sqlx::Error> {
        let workflows = sqlx::query_as::<_, Workflow>(
            r#"
            SELECT id, name, trigger_type, condition_operator, condition_value, watched_wallet, watched_token_mint, action_type, action_params, status, created_at
            FROM workflows
            WHERE trigger_type = ?
            ORDER BY created_at DESC
            LIMIT 30
            "#,
        )
        .bind(trigger_type.match_type())
        .fetch_all(&self.db)
        .await?;

        Ok(workflows)
    }

    pub async fn update_status(
        &self,
        id: i64,
        status: &str,
    ) -> Result<Option<Workflow>, sqlx::Error> {
        let workflow = sqlx::query_as::<_, Workflow>(
            r#"
            UPDATE workflows
            SET status = ?
            WHERE id = ?
            RETURNING id, name, trigger_type, condition_operator, condition_value, watched_wallet, watched_token_mint, action_type, action_params, status, created_at
            "#,
        )
        .bind(status)
        .bind(id)
        .fetch_optional(&self.db)
        .await?;

        Ok(workflow)
    }
}
