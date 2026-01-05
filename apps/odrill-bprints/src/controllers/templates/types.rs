//! Templates API - Types and common utilities

use crate::models::_entities::templates as templates_entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub q: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub id: i64,
    pub name: String,
    pub guid: String,
    pub display_name: String,
    pub description: Option<String>,
    pub tags: serde_json::Value,
    pub download_count: i64,
    pub user_id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub version: String,
}

impl From<templates_entity::Model> for TemplateResponse {
    fn from(t: templates_entity::Model) -> Self {
        Self {
            id: t.id,
            name: t.name,
            guid: t.guid.to_string(),
            display_name: t.display_name,
            description: t.description,
            tags: t.tags,
            download_count: t.download_count,
            user_id: t.user_id,
            created_at: t.created_at.to_string(),
            updated_at: t.updated_at.to_string(),
            version: t.version,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateParams {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}
