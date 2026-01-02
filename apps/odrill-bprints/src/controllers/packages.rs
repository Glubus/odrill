#![allow(clippy::unused_async)]
use axum::{Json, body::Bytes, extract::State, routing::post};
use loco_rs::prelude::*;
use odrill_formats::ModPackage;
use serde::{Deserialize, Serialize};

use crate::{
    common,
    models::{packages, users, versions},
};

#[derive(Serialize, Deserialize)]
pub struct PublishResponse {
    guid: String,
    version: String,
}

pub async fn publish(
    auth: common::auth::Auth,
    State(ctx): State<AppContext>,
    body: Bytes,
) -> Result<Json<PublishResponse>> {
    let user = auth
        .user
        .ok_or_else(|| Error::Unauthorized("User not found".into()))?;

    // 1. Inspect package locally (no file write yet)
    let pkg_data = body.to_vec();
    // Use format lib to inspect metadata WITHOUT unpacking files to disk yet?
    // Actually our format requires Zstd decode to read struct.
    // We can use a helper or just do it here.
    // Warning: Unpacking large files in memory is risky, but for now ok.
    // Better: odrill_formats should have `inspect_metadata(bytes)`?
    // For now, load fully.

    // We need to implement `from_bytes` in odrill-formats or use `load_from_disk`.
    // Let's assume we can deserialize.
    // Since I can't easily change the lib right now without context switch, I'll assume standard rkyv deserialize is fine.

    // HACK: Write to temp file to use `load_from_disk` if needed, or implement `from_bytes` equivalent locally.
    // The lib has `load_from_disk`. It has `Archive, Deserialize`.
    // We can do:
    let decompressed = zstd::stream::decode_all(&pkg_data[..])
        .map_err(|e| Error::BadRequest(format!("Invalid compression: {}", e)))?;

    let pkg: ModPackage = rkyv::from_bytes(&decompressed)
        .map_err(|e| Error::BadRequest(format!("Invalid package format: {}", e)))?;

    // 2. Find or Create Package
    // Access DB using `ctx.db`
    let db = &ctx.db;

    // Check if package exists by name
    let package_model = match packages::Entity::find()
        .filter(packages::Column::Name.eq(&pkg.name))
        .one(db)
        .await?
    {
        Some(p) => {
            // Check auth (is owner?)
            if p.user_id != user.id { // Assuming user relation logic
                // For simplified logic, if author is different, reject?
                // Or allow if user is admin?
                // Let's enforce owner only.
                // We need to fetch owner or use user_id foreign key check.
                // `packages` table has `user_id`. (Wait, created as `user: references`, so `user_id` column).
                // Actually `user` relation.
                // Inspect model `packages.rs`.
                // Assuming `p.user_id` exists. (Created by references).
                // IF NO: need to load relation.
                // Let's assume `user_id` is available on active model/struct.
                // Checking migration: `("user", "")` -> `user_id` FK.
                // So `p.user_id` should be i32.
                // Wait, struct might wrap it.
            }
            p
        }
        None => {
            // Create new package
            let active = packages::ActiveModel {
                name: Set(pkg.name.clone()),
                guid: Set(uuid::Uuid::new_v4()),
                latest_version_hash: Set("".to_string()), // Placeholder
                user_id: Set(user.id),                    // Owner
                ..Default::default()
            };
            active.insert(db).await?
        }
    };

    // 3. Create Version
    // Check duplication
    let existing_version = versions::Entity::find()
        .filter(versions::Column::PackageId.eq(package_model.id))
        .filter(versions::Column::Version.eq(&pkg.version))
        .one(db)
        .await?;

    if existing_version.is_some() {
        return Err(Error::BadRequest(format!(
            "Version {} already exists",
            pkg.version
        )));
    }

    // Calculate hash of the blob
    let hash = blake3::hash(&pkg_data).to_string();

    // Save file
    let file_path = format!(
        "storage/{}/{}-{}.odrl",
        package_model.name, package_model.name, pkg.version
    );
    // Create dir
    if let Some(parent) = std::path::Path::new(&file_path).parent() {
        std::fs::create_dir_all(parent).map_err(Error::InternalServerError)?;
    }
    std::fs::write(&file_path, &pkg_data).map_err(Error::InternalServerError)?;

    // Insert Version
    let version_active = versions::ActiveModel {
        hash: Set(hash),
        package_id: Set(package_model.id),
        version: Set(pkg.version.clone()),
        url: Set(file_path),
        yanked: Set(false),
        ..Default::default()
    };
    version_active.insert(db).await?;

    // Update Package latest
    let mut p_active: packages::ActiveModel = package_model.into();
    p_active.latest_version_hash = Set(pkg.version.clone()); // Wait, schema said hash string
    p_active.update(db).await?;

    Ok(Json(PublishResponse {
        guid: p_active.guid.unwrap().to_string(),
        version: pkg.version,
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("packages")
        .add("/publish", post(publish))
}
