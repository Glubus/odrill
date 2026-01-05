//! Package publish handlers - DB and file operations

use axum::body::Bytes;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use uuid::Uuid;

use crate::extractors::ApiKeyAuth;
use crate::models::_entities::{packages, versions};

pub async fn upsert_package(
    ctx: &AppContext,
    auth: &ApiKeyAuth,
    name: &str,
    version: &str,
) -> Result<(packages::Model, Uuid)> {
    let txn = ctx.db.begin().await?;

    let existing = packages::Entity::find()
        .filter(packages::Column::Name.eq(name))
        .one(&txn)
        .await?;

    let result = if let Some(p) = existing {
        if p.user_id != auth.user.id {
            return Err(Error::Unauthorized("Not owner".to_string()));
        }
        let guid = p.guid;
        let mut active: packages::ActiveModel = p.into();
        active.latest_version_hash = Set(version.to_string());
        active.updated_at = Set(chrono::Utc::now().into());
        let updated = active.update(&txn).await?;
        (updated, guid)
    } else {
        let guid = Uuid::new_v4();
        let new_pkg = packages::ActiveModel {
            name: Set(name.to_string()),
            guid: Set(guid),
            latest_version_hash: Set(version.to_string()),
            user_id: Set(auth.user.id),
            ..Default::default()
        };
        let inserted = new_pkg.insert(&txn).await?;
        (inserted, guid)
    };

    txn.commit().await?;
    Ok(result)
}

pub async fn create_version(
    ctx: &AppContext,
    package_id: i64,
    version: &str,
    body: &Bytes,
    guid: Uuid,
) -> Result<()> {
    let exists = versions::Entity::find()
        .filter(versions::Column::PackageId.eq(package_id))
        .filter(versions::Column::Version.eq(version))
        .one(&ctx.db)
        .await?;

    if exists.is_some() {
        return Err(Error::BadRequest(format!("Version {} exists", version)));
    }

    let filename = format!("{}_{}.odrl", guid, version);
    let new_ver = versions::ActiveModel {
        package_id: Set(package_id),
        version: Set(version.to_string()),
        hash: Set(blake3::hash(body).to_hex().to_string()),
        url: Set(format!("uploads/packages/{}", filename)),
        yanked: Set(false),
        ..Default::default()
    };
    new_ver.insert(&ctx.db).await?;
    Ok(())
}

pub async fn save_file(guid: Uuid, version: &str, body: &Bytes) -> Result<()> {
    let dir = std::path::Path::new("assets/uploads/packages");
    if !dir.exists() {
        tokio::fs::create_dir_all(dir).await.map_err(|e| {
            tracing::error!("Failed to create dir: {}", e);
            Error::InternalServerError
        })?;
    }

    let filename = format!("{}_{}.odrl", guid, version);
    let path = dir.join(&filename);

    tokio::fs::write(&path, body).await.map_err(|e| {
        tracing::error!("Failed to write file: {}", e);
        Error::InternalServerError
    })?;

    tracing::debug!(path = ?path, "Saved package file");
    Ok(())
}
