//! DELETE /api/templates/:guid - Delete a template (owner only)

use axum::Json;
use loco_rs::prelude::*;
use std::path::PathBuf;
use uuid::Uuid;

use crate::extractors::ApiKeyAuth;
use crate::models::{_entities::templates as templates_entity, templates::Model as TemplateModel};

pub async fn delete_template(
    State(ctx): State<AppContext>,
    auth: ApiKeyAuth,
    Path(guid): Path<String>,
) -> Result<impl IntoResponse> {
    auth.require_permission("templates:delete")
        .map_err(|_| Error::Unauthorized("Missing permission".to_string()))?;

    let uuid = Uuid::parse_str(&guid).map_err(|_| Error::BadRequest("Invalid GUID".to_string()))?;

    tracing::debug!(guid = %guid, user_id = auth.user.id, "Deleting template");

    let template = TemplateModel::find_by_guid(&ctx.db, uuid)
        .await
        .map_err(|_| Error::NotFound)?;

    if !template.is_owner(auth.user.id) {
        return Err(Error::Unauthorized("Not the owner".to_string()));
    }

    // Delete file
    let path = PathBuf::from(format!("assets{}", template.url));
    if path.exists() {
        let _ = std::fs::remove_file(&path);
        tracing::debug!(path = ?path, "Deleted template file");
    }

    templates_entity::Entity::delete_by_id(template.id)
        .exec(&ctx.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete: {}", e);
            Error::InternalServerError
        })?;

    format::json(())
}
