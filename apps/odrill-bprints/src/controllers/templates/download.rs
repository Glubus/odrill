//! GET /api/templates/:guid/download - Increment count and return URL

use axum::Json;
use loco_rs::prelude::*;
use uuid::Uuid;

use crate::models::templates::Model as TemplateModel;

pub async fn download(
    State(ctx): State<AppContext>,
    Path(guid): Path<String>,
) -> Result<impl IntoResponse> {
    let uuid = Uuid::parse_str(&guid).map_err(|_| Error::BadRequest("Invalid GUID".to_string()))?;

    tracing::debug!(guid = %guid, "Download request");

    let template = TemplateModel::find_by_guid(&ctx.db, uuid)
        .await
        .map_err(|_| Error::NotFound)?;

    // Increment download count
    if let Err(e) = template.increment_downloads(&ctx.db).await {
        tracing::warn!("Failed to increment download count: {}", e);
    }

    Ok(Json(serde_json::json!({ "url": template.url })))
}
