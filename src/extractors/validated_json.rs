use axum::extract::{FromRequest, Request};
use axum::Json;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::errors::AppError;

/// An Axum extractor that deserializes JSON and then runs `validator` checks.
/// Returns `AppError::BadRequest` on deserialization failure and
/// `AppError::Validation` if the payload fails validation rules.
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        value.validate()?;

        Ok(ValidatedJson(value))
    }
}
