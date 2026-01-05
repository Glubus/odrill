//! API Keys model - Key generation and validation logic.

use chrono::{DateTime, FixedOffset, Utc};
use loco_rs::prelude::*;
use rand::Rng;

use serde_json::Value as JsonValue;

pub use super::_entities::api_keys::{ActiveModel, Column, Entity, Model};

/// Generates a new random API key string.
/// Format: `odrill_sk_` followed by 32 random hex characters.
pub fn generate_key() -> String {
    let mut rng = rand::rng();
    let bytes: [u8; 16] = rng.random();
    format!("odrill_sk_{}", hex::encode(bytes))
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Find an API key by its secret string.
    pub async fn find_by_key(db: &DatabaseConnection, key: &str) -> ModelResult<Self> {
        Entity::find()
            .filter(Column::Key.eq(key))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Check if the key is still active (not expired).
    pub fn is_active(&self) -> bool {
        match self.expire_on.as_str() {
            "Never" => true,
            "Date" => {
                if let Some(expire_ts) = self.expire_value {
                    let now_ts = Utc::now().timestamp();
                    now_ts < expire_ts
                } else {
                    true // No value means no expiration
                }
            }
            "Usage" => {
                if let Some(max_usage) = self.expire_value {
                    self.usage_count < max_usage
                } else {
                    true
                }
            }
            _ => false, // Unknown type = expired
        }
    }

    /// Check if the key has a specific permission.
    pub fn has_permission(&self, perm: &str) -> bool {
        if let JsonValue::Array(perms) = &self.permissions {
            perms.iter().any(|p| p.as_str() == Some(perm))
        } else {
            false
        }
    }

    /// Increment usage count and update last_used_at.
    pub async fn record_usage(&self, db: &DatabaseConnection) -> ModelResult<()> {
        let now: DateTime<FixedOffset> = Utc::now().into();
        ActiveModel {
            id: ActiveValue::Unchanged(self.id),
            usage_count: ActiveValue::Set(self.usage_count + 1),
            last_used_at: ActiveValue::Set(Some(now)),
            ..Default::default()
        }
        .update(db)
        .await?;
        Ok(())
    }
}
