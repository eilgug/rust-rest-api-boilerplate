use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};

use crate::AppState;
use crate::errors::AppError;
use crate::extractors::auth::AuthUser;
use crate::routes::ProfileResponse;
use crate::services::user as user_service;

pub fn router() -> Router<AppState> {
    Router::new().route("/callback", post(auth_callback))
}

/// Called by the client right after a successful Supabase login.
/// Finds the existing profile or creates a new one (upsert by auth_id).
async fn auth_callback(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ProfileResponse>, AppError> {
    let profile = match user_service::find_by_auth_id(&state.db, &auth_user.id).await? {
        Some(existing) => existing,
        None => user_service::create_profile(&state.db, auth_user.id, auth_user.email).await?,
    };

    Ok(Json(profile.into()))
}
