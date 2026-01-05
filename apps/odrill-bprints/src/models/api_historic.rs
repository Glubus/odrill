//! API Historic model - Usage log for API keys.

use chrono::{DateTime, FixedOffset, Utc};
use loco_rs::prelude::*;

pub use super::_entities::api_historic::{ActiveModel, Column, Entity, Model};

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Log a usage event for an API key.
    pub async fn log(
        db: &DatabaseConnection,
        key_id: i64,
        action: &str,
        pkg_id: Option<i64>,
        ip_address: Option<String>,
    ) -> ModelResult<Self> {
        let now: DateTime<FixedOffset> = Utc::now().into();
        let record = ActiveModel {
            key_id: ActiveValue::Set(key_id),
            action: ActiveValue::Set(action.to_string()),
            pkg_id: ActiveValue::Set(pkg_id),
            created_at: ActiveValue::Set(now),
            ip_address: ActiveValue::Set(ip_address),
            ..Default::default()
        }
        .insert(db)
        .await?;
        Ok(record)
    }
}
