//! GET /api/packages - Package info endpoints

use axum::{Json, extract::Path};
use loco_rs::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct PackageInfo {
    pub name: String,
    pub latest_version: String,
}

#[derive(Serialize)]
pub struct VersionDetail {
    pub version: String,
    pub url: String,
}

pub async fn get_package(
    State(ctx): State<AppContext>,
    Path(name): Path<String>,
) -> Result<Json<PackageInfo>> {
    use crate::models::_entities::{packages, versions};
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

    let package = packages::Entity::find()
        .filter(packages::Column::Name.eq(&name))
        .one(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    // Get latest version (most recent upload)
    let latest = versions::Entity::find()
        .filter(versions::Column::PackageId.eq(package.id))
        .order_by_desc(versions::Column::CreatedAt)
        .one(&ctx.db)
        .await?;

    let latest_version = latest
        .map(|v| v.version)
        .unwrap_or_else(|| "0.0.0".to_string());

    Ok(Json(PackageInfo {
        name: package.name,
        latest_version,
    }))
}

pub async fn get_version(
    State(ctx): State<AppContext>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<VersionDetail>> {
    use crate::models::_entities::{package_downloads, packages, versions};
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};

    // Find package and specific version
    // Using transaction to ensure counters and history are atomic
    let txn = ctx.db.begin().await?;

    let package = packages::Entity::find()
        .filter(packages::Column::Name.eq(&name))
        .one(&txn)
        .await?
        .ok_or(Error::NotFound)?;

    let version_model = versions::Entity::find()
        .filter(versions::Column::PackageId.eq(package.id))
        .filter(versions::Column::Version.eq(&version))
        .one(&txn)
        .await?
        .ok_or(Error::NotFound)?;

    // Increment package downloads
    let mut pkg_active: packages::ActiveModel = package.clone().into();
    pkg_active.downloads = Set(package.downloads + 1);
    pkg_active.update(&txn).await?;

    // Increment version downloads
    let mut ver_active: versions::ActiveModel = version_model.clone().into();
    ver_active.downloads = Set(version_model.downloads + 1);
    ver_active.update(&txn).await?;

    // Log history
    package_downloads::ActiveModel {
        package_id: Set(package.id),
        version_id: Set(version_model.id),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    txn.commit().await?;

    Ok(Json(VersionDetail {
        version: version_model.version,
        url: version_model.url,
    }))
}
