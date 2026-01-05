//! Templates page controllers

use axum::{extract::Query, response::Html};
use axum_extra::extract::cookie::CookieJar;
use loco_rs::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::templates::Model as TemplateModel;

#[derive(Debug, Deserialize, Default)]
pub struct ListParams {
    pub q: Option<String>,
}

/// GET /templates - Templates listing page
pub async fn index(
    State(ctx): State<AppContext>,
    jar: CookieJar,
    ViewEngine(v): ViewEngine<TeraView>,
    Query(params): Query<ListParams>,
) -> Result<impl IntoResponse> {
    use crate::models::_entities::users;
    use crate::models::users::Model as UserModel;
    use sea_orm::EntityTrait;

    let user = UserModel::from_cookie(&ctx, &jar).await;

    let templates = TemplateModel::search(&ctx.db, params.q.as_deref(), 50, 0)
        .await
        .unwrap_or_default();

    // Fetch all users for creator names
    let user_ids: Vec<i64> = templates.iter().map(|t| t.user_id).collect();
    let all_users: std::collections::HashMap<i64, String> = users::Entity::find()
        .all(&ctx.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|u| user_ids.contains(&u.id))
        .map(|u| (u.id, u.name))
        .collect();

    // Convert templates with creator info
    let templates_data: Vec<serde_json::Value> = templates
        .into_iter()
        .map(|t| {
            let tags: Vec<String> = serde_json::from_value(t.tags.clone()).unwrap_or_default();
            let creator = all_users.get(&t.user_id).cloned().unwrap_or_default();
            serde_json::json!({
                "id": t.id,
                "guid": t.guid.to_string(),
                "display_name": t.display_name,
                "description": t.description,
                "tags": tags,
                "download_count": t.download_count,
                "version": t.version,
                "creator": creator,
            })
        })
        .collect();

    let html = v.render(
        "templates/index.html",
        data!({
            "user": user,
            "templates": templates_data,
            "query": params.q.unwrap_or_default(),
        }),
    )?;
    Ok(Html(html.to_string()))
}

/// GET /templates/:guid - Template detail page
pub async fn show(
    State(ctx): State<AppContext>,
    jar: CookieJar,
    ViewEngine(v): ViewEngine<TeraView>,
    Path(guid): Path<String>,
) -> Result<impl IntoResponse> {
    use crate::models::users::Model as UserModel;
    let user = UserModel::from_cookie(&ctx, &jar).await;

    let uuid = Uuid::parse_str(&guid).map_err(|_| Error::BadRequest("Invalid GUID".to_string()))?;

    let template = TemplateModel::find_by_guid(&ctx.db, uuid)
        .await
        .map_err(|_| Error::NotFound)?;

    let tags: Vec<String> = serde_json::from_value(template.tags.clone()).unwrap_or_default();

    let html = v.render(
        "templates/show.html",
        data!({
            "user": user,
            "template": {
                "id": template.id,
                "name": template.name,
                "guid": template.guid.to_string(),
                "display_name": template.display_name,
                "description": template.description,
                "tags": tags,
                "download_count": template.download_count,
                "updated_at": template.updated_at.format("%Y-%m-%d").to_string(),
            },
        }),
    )?;
    Ok(Html(html.to_string()))
}
