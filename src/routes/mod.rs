pub mod auth;
pub mod user;

use axum::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::models::profile;

/// Shared response DTO for profile data returned to clients.
#[derive(Serialize)]
pub struct ProfileResponse {
    pub id: Uuid,
    pub auth_id: String,
    pub display_name: Option<String>,
    pub email: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
}

impl From<profile::Model> for ProfileResponse {
    fn from(m: profile::Model) -> Self {
        ProfileResponse {
            id: m.id,
            auth_id: m.auth_id,
            display_name: m.display_name,
            email: m.email,
            avatar_url: m.avatar_url,
            bio: m.bio,
        }
    }
}

/// GET /health -- lightweight check that the DB connection is alive.
pub async fn health(
    axum::extract::State(state): axum::extract::State<crate::AppState>,
) -> Result<Json<serde_json::Value>, crate::errors::AppError> {
    use sea_orm::ConnectionTrait;

    state
        .db
        .execute_unprepared("SELECT 1")
        .await
        .map_err(|e| crate::errors::AppError::Internal(format!("db health check failed: {e}")))?;

    Ok(Json(serde_json::json!({ "status": "ok" })))
}
