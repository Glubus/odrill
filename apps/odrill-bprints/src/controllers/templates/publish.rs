//! POST /api/templates/publish - Publish a template

use axum::{Json, extract::Multipart};
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::ActiveValue;
use std::path::PathBuf;
use uuid::Uuid;

use super::{security, types::TemplateResponse};
use crate::extractors::ApiKeyAuth;
use crate::models::{_entities::templates as templates_entity, templates::Model as TemplateModel};

pub async fn publish(
    State(ctx): State<AppContext>,
    auth: ApiKeyAuth,
    mut multipart: Multipart,
) -> Result<Json<TemplateResponse>> {
    auth.require_permission("publish")
        .map_err(|_| Error::Unauthorized("Missing permission".to_string()))?;

    // 1. Extract file from multipart
    let file_bytes = extract_file(&mut multipart).await?;

    // 2. Decode and validate
    let hash = blake3::hash(&file_bytes).to_hex().to_string();
    let pkg = container::decode(&file_bytes)
        .map_err(|e| Error::BadRequest(format!("Invalid format: {}", e)))?;

    tracing::debug!(name = %pkg.name, version = %pkg.version, "Publishing template");

    // 3. Security validation
    match security::validate_package(&pkg) {
        Err(critical) => {
            return Err(Error::BadRequest(format!("Rejected: {:?}", critical)));
        }
        Ok(warnings) if !warnings.is_empty() => {
            tracing::warn!("Template '{}' has warnings: {:?}", pkg.name, warnings);
        }
        _ => {}
    }

    // 4. DB upsert
    let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
    let template = build_template_model(&ctx, &auth, &pkg, &hash, now).await?;

    // 5. Save file
    let guid = extract_guid(&template)?;
    save_template_file(&file_bytes, guid).await?;

    // 6. Update URL and save
    let mut final_active = template;
    final_active.url = ActiveValue::Set(format!("/uploads/templates/{}.rkyv.zstd", guid));

    let _saved = final_active.save(&ctx.db).await.map_err(|e| {
        tracing::error!("DB save failed: {}", e);
        Error::InternalServerError
    })?;

    let reloaded = TemplateModel::find_by_guid(&ctx.db, guid)
        .await
        .map_err(|_| Error::InternalServerError)?;

    Ok(Json(reloaded.into()))
}

async fn extract_file(multipart: &mut Multipart) -> Result<Vec<u8>> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| Error::BadRequest("Invalid multipart".to_string()))?
    {
        if field.name() == Some("file") {
            return field
                .bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|_| Error::BadRequest("Failed to read".to_string()));
        }
    }
    Err(Error::BadRequest("No file uploaded".to_string()))
}

async fn build_template_model(
    ctx: &AppContext,
    auth: &ApiKeyAuth,
    pkg: &pkg::ModPackage,
    hash: &str,
    now: chrono::DateTime<chrono::FixedOffset>,
) -> Result<templates_entity::ActiveModel> {
    let existing = TemplateModel::find_by_name(&ctx.db, &pkg.name).await;

    match existing {
        Ok(t) => {
            if !t.is_owner(auth.user.id) {
                return Err(Error::Unauthorized("Not owner".to_string()));
            }
            if pkg.version.as_str() < t.version.as_str() {
                return Err(Error::BadRequest("Version too old".to_string()));
            }
            let mut active: templates_entity::ActiveModel = t.into();
            active.version = ActiveValue::Set(pkg.version.clone());
            active.hash = ActiveValue::Set(hash.to_string());
            active.updated_at = ActiveValue::Set(now);
            Ok(active)
        }
        Err(_) => Ok(templates_entity::ActiveModel {
            name: ActiveValue::Set(pkg.name.clone()),
            guid: ActiveValue::Set(Uuid::new_v4()),
            display_name: ActiveValue::Set(pkg.name.clone()),
            description: ActiveValue::Set(None),
            tags: ActiveValue::Set(serde_json::json!([])),
            download_count: ActiveValue::Set(0),
            hash: ActiveValue::Set(hash.to_string()),
            url: ActiveValue::Set(String::new()),
            version: ActiveValue::Set(pkg.version.clone()),
            user_id: ActiveValue::Set(auth.user.id),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
            ..Default::default()
        }),
    }
}

fn extract_guid(template: &templates_entity::ActiveModel) -> Result<Uuid> {
    match &template.guid {
        ActiveValue::Set(g) | ActiveValue::Unchanged(g) => Ok(*g),
        _ => Err(Error::InternalServerError),
    }
}

async fn save_template_file(bytes: &[u8], guid: Uuid) -> Result<()> {
    let path = PathBuf::from("assets/uploads/templates");
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p).map_err(|_| Error::InternalServerError)?;
    }
    std::fs::create_dir_all(&path).map_err(|_| Error::InternalServerError)?;

    let file_path = path.join(format!("{}.rkyv.zstd", guid));
    std::fs::write(&file_path, bytes).map_err(|e| {
        tracing::error!("File write failed: {}", e);
        Error::InternalServerError
    })?;

    tracing::debug!(path = ?file_path, "Saved template file");
    Ok(())
}
