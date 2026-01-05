//! SSR controller for packages browser - Temporarily stubbed

use axum::{
    extract::{Path, Query},
    response::Html,
};
use loco_rs::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct SearchQuery {
    #[allow(dead_code)]
    q: Option<String>,
}

use axum_extra::extract::cookie::CookieJar;

pub async fn index(
    State(ctx): State<AppContext>,
    jar: CookieJar,
    ViewEngine(v): ViewEngine<TeraView>,
    Query(query): Query<SearchQuery>,
) -> Result<impl IntoResponse> {
    use crate::models::_entities::packages;
    use crate::models::users::Model as UserModel;
    use sea_orm::{EntityTrait, QueryOrder};

    let user = UserModel::from_cookie(&ctx, &jar).await;

    // Fetch packages from DB
    let packages_list = packages::Entity::find()
        .order_by_desc(packages::Column::UpdatedAt)
        .all(&ctx.db)
        .await
        .unwrap_or_default();

    let packages_data: Vec<serde_json::Value> = packages_list
        .into_iter()
        .map(|p| {
            serde_json::json!({
                "name": p.name,
                "guid": p.guid.to_string(),
                "latest_version": p.latest_version_hash,
                "downloads": 0,
            })
        })
        .collect();

    let html = v.render(
        "packages/index.html",
        data!({
            "packages": packages_data,
            "query": query.q.unwrap_or_default(),
            "user": user
        }),
    )?;
    Ok(Html(html.to_string()))
}

pub async fn show(
    State(ctx): State<AppContext>,
    jar: CookieJar,
    ViewEngine(v): ViewEngine<TeraView>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse> {
    use crate::models::_entities::{package_downloads, packages, versions};
    use crate::models::users::Model as UserModel;
    use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

    let user = UserModel::from_cookie(&ctx, &jar).await;

    // Fetch package by name
    let package = packages::Entity::find()
        .filter(packages::Column::Name.eq(&name))
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?
        .ok_or_else(|| Error::NotFound)?;

    // Fetch versions for this package
    let versions_list = versions::Entity::find()
        .filter(versions::Column::PackageId.eq(package.id))
        .order_by_desc(versions::Column::CreatedAt)
        .all(&ctx.db)
        .await
        .unwrap_or_default();

    let versions_data: Vec<serde_json::Value> = versions_list
        .into_iter()
        .map(|v| {
            serde_json::json!({
                "version": v.version,
                "yanked": v.yanked,
                "created_at": v.created_at.format("%Y-%m-%d").to_string(),
            })
        })
        .collect();

    // Fetch download history (Cached)
    let cache_key = format!("pkg_stats_90d_{}", package.id);
    let download_history: Vec<serde_json::Value> =
        if let Ok(Some(cached)) = ctx.cache.get(&cache_key).await {
            cached
        } else {
            // Fetch download history for the last 90 days
            let now = chrono::Utc::now();
            let ninety_days_ago = now - chrono::Duration::days(90);

            let history_entries: Vec<(String, i64)> = package_downloads::Entity::find()
                .filter(package_downloads::Column::PackageId.eq(package.id))
                .filter(package_downloads::Column::CreatedAt.gte(ninety_days_ago))
                .select_only()
                .column(package_downloads::Column::CreatedAt)
                .column_as(package_downloads::Column::Id.count(), "count")
                .group_by(package_downloads::Column::CreatedAt)
                .into_tuple::<(chrono::DateTime<chrono::FixedOffset>, i64)>()
                .all(&ctx.db)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|(date, count)| (date.format("%Y-%m-%d").to_string(), count))
                .collect();

            // Fill gaps
            let history_map: std::collections::HashMap<String, i64> =
                history_entries.into_iter().collect();
            let result: Vec<serde_json::Value> = (0..90)
                .map(|i| {
                    let date = chrono::Local::now() - chrono::Duration::days(89 - i);
                    let date_str = date.format("%Y-%m-%d").to_string();
                    let downloads = *history_map.get(&date_str).unwrap_or(&0);
                    serde_json::json!({
                        "date": date_str,
                        "downloads": downloads
                    })
                })
                .collect();

            // Cache for 1 hour (3600 seconds)
            if let Err(e) = ctx.cache.insert(&cache_key, &result).await {
                tracing::error!("Failed to cache download stats: {}", e);
            }
            result
        };

    let html = v.render(
        "packages/show.html",
        data!({
            "user": user,
            "package": {
                "name": package.name,
                "guid": package.guid.to_string(),
                "latest_version": package.latest_version_hash,
                "downloads": package.downloads, // Use real count
            },
            "versions": versions_data,
            "download_history": download_history
        }),
    )?;
    Ok(Html(html.to_string()))
}
