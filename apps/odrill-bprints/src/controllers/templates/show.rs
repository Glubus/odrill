//! GET /api/templates/:guid - Get template by GUID

use axum::Json;
use loco_rs::prelude::*;
use uuid::Uuid;

use super::types::TemplateResponse;
use crate::models::templates::Model as TemplateModel;

pub async fn show(
    State(ctx): State<AppContext>,
    Path(guid): Path<String>,
) -> Result<Json<TemplateResponse>> {
    tracing::debug!(guid = %guid, "Fetching template");

    let uuid = Uuid::parse_str(&guid).map_err(|_| Error::BadRequest("Invalid GUID".to_string()))?;

    let template = TemplateModel::find_by_guid(&ctx.db, uuid)
        .await
        .map_err(|_| Error::NotFound)?;

    Ok(Json(template.into()))
}
