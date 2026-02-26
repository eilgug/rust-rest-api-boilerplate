use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::Deserialize;

use crate::AppState;
use crate::errors::AppError;

/// Claims embedded in a Supabase-issued JWT.
#[derive(Debug, Deserialize)]
struct SupabaseClaims {
    sub: String,
    email: Option<String>,
    role: Option<String>,
}

/// Extractor that validates the Supabase JWT from the Authorization header
/// and makes the authenticated user's identity available to handlers.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    #[allow(dead_code)]
    pub role: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("missing authorization header".into()))?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("invalid authorization format".into()))?;

        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["authenticated"]);

        let token_data = decode::<SupabaseClaims>(
            token,
            &DecodingKey::from_secret(state.config.supabase_jwt_secret.as_bytes()),
            &validation,
        )
        .map_err(|e| AppError::Unauthorized(format!("invalid token: {e}")))?;

        let claims = token_data.claims;

        Ok(AuthUser {
            id: claims.sub,
            email: claims.email.unwrap_or_default(),
            role: claims.role.unwrap_or_else(|| "authenticated".into()),
        })
    }
}
