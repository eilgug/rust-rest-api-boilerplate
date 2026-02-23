use axum::extract::{Path, State};
use axum::routing::{delete, get, put};
use axum::{Json, Router};
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::extractors::auth::AuthUser;
use crate::extractors::validated_json::ValidatedJson;
use crate::routes::ProfileResponse;
use crate::services::user as user_service;
use crate::AppState;

#[derive(serde::Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min = 2, max = 100))]
    pub display_name: Option<String>,
    #[validate(length(max = 500))]
    pub bio: Option<String>,
    #[validate(length(max = 2048))]
    pub avatar_url: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_me))
        .route("/me", put(update_me))
        .route("/me", delete(delete_me))
        .route("/{id}", get(get_by_id))
}

async fn get_me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ProfileResponse>, AppError> {
    let profile = user_service::find_by_auth_id(&state.db, &auth_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("profile not found".into()))?;

    Ok(Json(profile.into()))
}

async fn update_me(
    State(state): State<AppState>,
    auth_user: AuthUser,
    ValidatedJson(body): ValidatedJson<UpdateProfileRequest>,
) -> Result<Json<ProfileResponse>, AppError> {
    let profile = user_service::update_profile(
        &state.db,
        &auth_user.id,
        body.display_name,
        body.bio,
        body.avatar_url,
    )
    .await?;

    Ok(Json(profile.into()))
}

async fn delete_me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<axum::http::StatusCode, AppError> {
    user_service::delete_profile(&state.db, &auth_user.id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn get_by_id(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ProfileResponse>, AppError> {
    let profile = user_service::find_by_id(&state.db, id)
        .await?
        .ok_or_else(|| AppError::NotFound("profile not found".into()))?;

    Ok(Json(profile.into()))
}
