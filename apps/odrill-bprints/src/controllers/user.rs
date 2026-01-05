use crate::models::_entities::users;
use crate::models::{_entities::api_keys as api_keys_entity, api_keys};
use axum::{
    Json,
    extract::{Multipart, State},
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use chrono::Utc;
use image::GenericImageView;
use loco_rs::prelude::*;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct CropParams {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyParams {
    pub name: String,
    pub permissions: Vec<String>,
    pub expire_on: String, // "Never", "Date", "Usage"
    pub expire_value: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: i64,
    pub name: String,
    pub key: Option<String>, // Only set on creation
    pub permissions: serde_json::Value,
    pub expire_on: String,
    pub expire_value: Option<i64>,
    pub usage_count: i64,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

async fn get_user_from_cookie(ctx: &AppContext, jar: &CookieJar) -> Result<users::Model> {
    if let Some(token) = jar.get("odrill_token") {
        if let Some(auth_config) = ctx.config.auth.as_ref() {
            let secret = &auth_config.jwt.as_ref().unwrap().secret;
            let validation = jsonwebtoken::Validation::default();

            if let Ok(data) = jsonwebtoken::decode::<serde_json::Value>(
                token.value(),
                &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
                &validation,
            ) {
                let pid = data.claims["pid"].as_str().unwrap_or_default();
                if let Ok(user) = users::Model::find_by_pid(&ctx.db, pid).await {
                    return Ok(user);
                }
            }
        }
    }
    Err(Error::Unauthorized("User not found".to_string()))
}

pub async fn upload_avatar(
    jar: CookieJar,
    State(ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut x = 0;
    let mut y = 0;
    let mut width = 0;
    let mut height = 0;

    // Parse multipart
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| Error::BadRequest(e.to_string()))?
    {
        let name = field.name().unwrap_or_default().to_string();

        if name == "file" {
            let data = match field.bytes().await {
                Ok(d) => d,
                Err(e) => return bad_request(e.to_string()),
            };
            file_bytes = Some(data.to_vec());
        } else if name == "x" {
            let val = match field.text().await {
                Ok(t) => t,
                Err(e) => return bad_request(e.to_string()),
            };
            x = val.parse().unwrap_or(0);
        } else if name == "y" {
            let val = match field.text().await {
                Ok(t) => t,
                Err(e) => return bad_request(e.to_string()),
            };
            y = val.parse().unwrap_or(0);
        } else if name == "width" {
            let val = match field.text().await {
                Ok(t) => t,
                Err(e) => return bad_request(e.to_string()),
            };
            width = val.parse().unwrap_or(0);
        } else if name == "height" {
            let val = match field.text().await {
                Ok(t) => t,
                Err(e) => return bad_request(e.to_string()),
            };
            height = val.parse().unwrap_or(0);
        }
    }

    // Authenticate via cookie
    let user = get_user_from_cookie(&ctx, &jar).await?;

    let Some(bytes) = file_bytes else {
        return bad_request("No file provided");
    };

    if width == 0 || height == 0 {
        return bad_request("Invalid dimensions");
    }

    // Image processing
    let img = match image::load_from_memory(&bytes) {
        Ok(i) => i,
        Err(e) => return bad_request(format!("Invalid image: {}", e)),
    };

    // Crop
    let mut cropped = img.crop_imm(x, y, width, height);

    // Force square
    let (c_w, c_h) = cropped.dimensions();
    if c_w != c_h {
        let size = std::cmp::min(c_w, c_h);
        cropped = cropped.crop_imm(0, 0, size, size);
    }

    // Resize to 256x256
    let resized = cropped.resize(256, 256, image::imageops::FilterType::Lanczos3);

    // Save
    let uploads_dir = PathBuf::from("assets/uploads/avatars");
    if !uploads_dir.exists() {
        if let Err(_e) = std::fs::create_dir_all(&uploads_dir) {
            return Err(Error::InternalServerError);
        }
    }

    let filename = format!("{}.webp", user.pid);
    let path = uploads_dir.join(filename);

    // Encode as WebP
    if let Err(e) = resized.save_with_format(&path, image::ImageFormat::WebP) {
        return bad_request(format!("Failed to save image: {}", e));
    }

    format::json(())
}

/// GET /api/user/api-keys - List all API keys for the current user
pub async fn list_api_keys(
    jar: CookieJar,
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<ApiKeyResponse>>> {
    let user = get_user_from_cookie(&ctx, &jar).await?;

    let keys = api_keys_entity::Entity::find()
        .filter(api_keys_entity::Column::UserId.eq(user.id))
        .all(&ctx.db)
        .await
        .map_err(|_e| Error::InternalServerError)?;

    let response: Vec<ApiKeyResponse> = keys
        .into_iter()
        .map(|k| ApiKeyResponse {
            id: k.id,
            name: k.name,
            key: None, // Never expose the key after creation
            permissions: k.permissions,
            expire_on: k.expire_on,
            expire_value: k.expire_value,
            usage_count: k.usage_count,
            created_at: k.created_at.to_string(),
            last_used_at: k.last_used_at.map(|t| t.to_string()),
        })
        .collect();

    Ok(Json(response))
}

/// POST /api/user/api-keys - Create a new API key
pub async fn create_api_key(
    jar: CookieJar,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateApiKeyParams>,
) -> Result<Json<ApiKeyResponse>> {
    let user = get_user_from_cookie(&ctx, &jar).await?;

    // Generate a new key
    let key_string = api_keys::generate_key();
    let now: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();

    let new_key = api_keys_entity::ActiveModel {
        user_id: ActiveValue::Set(user.id),
        key: ActiveValue::Set(key_string.clone()),
        name: ActiveValue::Set(params.name.clone()),
        permissions: ActiveValue::Set(serde_json::json!(params.permissions)),
        expire_on: ActiveValue::Set(params.expire_on.clone()),
        expire_value: ActiveValue::Set(params.expire_value),
        usage_count: ActiveValue::Set(0),
        created_at: ActiveValue::Set(now),
        last_used_at: ActiveValue::Set(None),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|_e| Error::InternalServerError)?;

    Ok(Json(ApiKeyResponse {
        id: new_key.id,
        name: new_key.name,
        key: Some(key_string), // Only expose the key on creation!
        permissions: new_key.permissions,
        expire_on: new_key.expire_on,
        expire_value: new_key.expire_value,
        usage_count: new_key.usage_count,
        created_at: new_key.created_at.to_string(),
        last_used_at: None,
    }))
}

/// DELETE /api/user/api-keys/:id - Revoke an API key
pub async fn delete_api_key(
    jar: CookieJar,
    State(ctx): State<AppContext>,
    Path(key_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let user = get_user_from_cookie(&ctx, &jar).await?;

    // Find the key and verify ownership
    let key = api_keys_entity::Entity::find_by_id(key_id)
        .one(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?
        .ok_or_else(|| Error::NotFound)?;

    if key.user_id != user.id {
        return Err(Error::Unauthorized("Not your key".to_string()));
    }

    // Delete it
    api_keys_entity::Entity::delete_by_id(key_id)
        .exec(&ctx.db)
        .await
        .map_err(|_| Error::InternalServerError)?;

    format::json(())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/user")
        .add("/avatar", post(upload_avatar))
        .add("/api-keys", get(list_api_keys))
        .add("/api-keys", post(create_api_key))
        .add("/api-keys/{id}", delete(delete_api_key))
        .add("/", get(current))
}

use crate::extractors::ApiKeyAuth;
use crate::views::auth::CurrentResponse;

pub async fn current(
    auth: ApiKeyAuth,
    State(_ctx): State<AppContext>,
) -> Result<Json<CurrentResponse>> {
    Ok(Json(CurrentResponse::new(&auth.user)))
}
