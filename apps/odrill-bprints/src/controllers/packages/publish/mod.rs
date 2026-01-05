//! POST /api/packages/publish - Publish a new package version

mod handlers;
mod security;

use axum::{Json, body::Bytes};
use loco_rs::prelude::*;
use serde::Serialize;

use crate::extractors::ApiKeyAuth;
use crate::models::api_historic;

#[derive(Serialize)]
pub struct PublishResponse {
    pub guid: String,
    pub version: String,
}

pub async fn handler(
    State(ctx): State<AppContext>,
    auth: ApiKeyAuth,
    body: Bytes,
) -> Result<Json<PublishResponse>> {
    auth.require_permission("publish")
        .map_err(|_| Error::Unauthorized("Missing permission".to_string()))?;

    tracing::debug!(user_id = auth.user.id, key_id = auth.key.id, "Publishing");

    // 1. Decode
    let pkg = container::decode(&body).map_err(|e| {
        tracing::error!("Decode failed: {}", e);
        Error::BadRequest("Invalid format".to_string())
    })?;

    // 2. Security validation
    security::validate(&pkg)?;

    // 3. DB upsert
    let (pkg_model, guid) = handlers::upsert_package(&ctx, &auth, &pkg.name, &pkg.version).await?;

    // 4. Create version
    handlers::create_version(&ctx, pkg_model.id, &pkg.version, &body, guid).await?;

    // 5. Save file
    handlers::save_file(guid, &pkg.version, &body).await?;

    // 6. Log
    if let Err(e) =
        api_historic::Model::log(&ctx.db, auth.key.id, "publish", Some(pkg_model.id), None).await
    {
        tracing::warn!("Log failed: {}", e);
    }

    Ok(Json(PublishResponse {
        guid: guid.to_string(),
        version: pkg.version,
    }))
}
