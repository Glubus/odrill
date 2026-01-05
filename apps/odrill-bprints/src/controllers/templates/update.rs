//! PUT /api/templates/:guid - Update metadata (owner only)

use axum::Json;
use loco_rs::prelude::*;
use sea_orm::ActiveValue;
use uuid::Uuid;

use super::types::{TemplateResponse, UpdateTemplateParams};
use crate::extractors::ApiKeyAuth;
use crate::models::{_entities::templates as templates_entity, templates::Model as TemplateModel};

pub async fn update(
    State(ctx): State<AppContext>,
    auth: ApiKeyAuth,
    Path(guid): Path<String>,
    Json(params): Json<UpdateTemplateParams>,
) -> Result<Json<TemplateResponse>> {
    auth.require_permission("publish")
        .map_err(|_| Error::Unauthorized("Missing permission".to_string()))?;

    let uuid = Uuid::parse_str(&guid).map_err(|_| Error::BadRequest("Invalid GUID".to_string()))?;

    tracing::debug!(guid = %guid, user_id = auth.user.id, "Updating template");

    let template = TemplateModel::find_by_guid(&ctx.db, uuid)
        .await
        .map_err(|_| Error::NotFound)?;

    if !template.is_owner(auth.user.id) {
        return Err(Error::Unauthorized("Not the owner".to_string()));
    }

    let mut active: templates_entity::ActiveModel = template.into();

    if let Some(display_name) = params.display_name {
        active.display_name = ActiveValue::Set(display_name);
    }
    if let Some(description) = params.description {
        active.description = ActiveValue::Set(Some(description));
    }
    if let Some(tags) = params.tags {
        active.tags = ActiveValue::Set(serde_json::json!(tags));
    }

    let updated = active.update(&ctx.db).await.map_err(|e| {
        tracing::error!("Update failed: {}", e);
        Error::InternalServerError
    })?;

    Ok(Json(updated.into()))
}
