//! API Key authentication extractor.
//!
//! This extractor validates `Authorization: Bearer <API_KEY>` headers
//! and provides the authenticated User and ApiKey to handlers.

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
    response::{IntoResponse, Response},
};
use loco_rs::prelude::*;

use crate::models::{
    _entities::api_keys::Model as ApiKeyModel, _entities::users::Model as UserModel, api_keys,
    users,
};

/// Error type for API Key authentication failures.
#[derive(Debug)]
pub enum ApiKeyError {
    MissingHeader,
    InvalidFormat,
    KeyNotFound,
    KeyExpired,
    InsufficientPermissions,
    DatabaseError(String),
}

impl IntoResponse for ApiKeyError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::MissingHeader => (StatusCode::UNAUTHORIZED, "Missing Authorization header"),
            Self::InvalidFormat => (StatusCode::UNAUTHORIZED, "Invalid Authorization format"),
            Self::KeyNotFound => (StatusCode::UNAUTHORIZED, "API Key not found"),
            Self::KeyExpired => (StatusCode::UNAUTHORIZED, "API Key has expired"),
            Self::InsufficientPermissions => (StatusCode::FORBIDDEN, "Insufficient permissions"),
            Self::DatabaseError(e) => {
                tracing::error!("Database error in API key auth: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };
        (status, message).into_response()
    }
}

/// Extractor that validates an API Key from the Authorization header.
///
/// Use this in handler signatures for routes that require API Key auth:
/// ```ignore
/// async fn handler(auth: ApiKeyAuth, ...) -> Result<...> {
///     let user = auth.user;
///     let key = auth.key;
/// }
/// ```
pub struct ApiKeyAuth {
    pub user: UserModel,
    pub key: ApiKeyModel,
}

impl<S> FromRequestParts<S> for ApiKeyAuth
where
    S: Send + Sync,
    AppContext: FromRef<S>,
{
    type Rejection = ApiKeyError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Get the AppContext from state
        let ctx = AppContext::from_ref(state);
        let db = &ctx.db;

        // Extract Authorization header
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(ApiKeyError::MissingHeader)?;

        // Must be "Bearer <key>"
        let key_str = auth_header
            .strip_prefix("Bearer ")
            .or_else(|| auth_header.strip_prefix("bearer "))
            .ok_or(ApiKeyError::InvalidFormat)?;

        // Find the API key in database
        let api_key = api_keys::Model::find_by_key(db, key_str)
            .await
            .map_err(|_| ApiKeyError::KeyNotFound)?;

        // Check if key is still active
        if !api_key.is_active() {
            return Err(ApiKeyError::KeyExpired);
        }

        // Find the user associated with this key
        let user = users::Model::find_by_id(db, api_key.user_id)
            .await
            .map_err(|e| ApiKeyError::DatabaseError(e.to_string()))?;

        // Record usage (non-blocking, fire-and-forget style would be better but keep simple)
        if let Err(e) = api_key.record_usage(db).await {
            tracing::warn!("Failed to record API key usage: {}", e);
        }

        Ok(Self { user, key: api_key })
    }
}

impl ApiKeyAuth {
    /// Check if the authenticated key has a specific permission.
    pub fn require_permission(&self, perm: &str) -> Result<(), ApiKeyError> {
        if self.key.has_permission(perm) {
            Ok(())
        } else {
            Err(ApiKeyError::InsufficientPermissions)
        }
    }
}
