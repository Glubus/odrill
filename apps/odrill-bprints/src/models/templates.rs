//! Templates model with helper methods.

use sea_orm::{ActiveValue, Condition, QueryOrder, QuerySelect, entity::prelude::*};
use uuid::Uuid;

pub use super::_entities::templates::{ActiveModel, Column, Entity, Model};
pub type Templates = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl Model {
    /// Find a template by its GUID.
    pub async fn find_by_guid(db: &DatabaseConnection, guid: Uuid) -> Result<Self, DbErr> {
        Entity::find()
            .filter(Column::Guid.eq(guid))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Template not found".to_string()))
    }

    /// Find a template by its name.
    pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> Result<Self, DbErr> {
        Entity::find()
            .filter(Column::Name.eq(name))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Template not found".to_string()))
    }

    /// Search templates by query.
    pub async fn search(
        db: &DatabaseConnection,
        query: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Self>, DbErr> {
        let mut q = Entity::find().order_by_desc(Column::DownloadCount);

        if let Some(search) = query {
            q = q.filter(
                Condition::any()
                    .add(Column::Name.contains(search))
                    .add(Column::DisplayName.contains(search))
                    .add(Column::Description.contains(search)),
            );
        }

        q.limit(limit).offset(offset).all(db).await
    }

    /// Increment download count.
    pub async fn increment_downloads(&self, db: &DatabaseConnection) -> Result<(), DbErr> {
        let mut active: ActiveModel = self.clone().into();
        active.download_count = ActiveValue::Set(self.download_count + 1);
        active.update(db).await?;
        Ok(())
    }

    /// Check if user owns this template.
    pub fn is_owner(&self, user_id: i64) -> bool {
        self.user_id == user_id
    }
}
