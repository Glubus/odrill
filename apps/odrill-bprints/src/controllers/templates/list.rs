//! GET /api/templates - List/search templates

use axum::{Json, extract::Query};
use loco_rs::prelude::*;

use super::types::{ListParams, TemplateResponse};
use crate::models::templates::Model as TemplateModel;

pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<TemplateResponse>>> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    tracing::debug!(q = ?params.q, limit, offset, "Listing templates");

    let templates = TemplateModel::search(&ctx.db, params.q.as_deref(), limit, offset)
        .await
        .map_err(|e| {
            tracing::error!("Search failed: {}", e);
            Error::InternalServerError
        })?;

    let response: Vec<TemplateResponse> = templates.into_iter().map(Into::into).collect();
    Ok(Json(response))
}
