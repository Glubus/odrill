pub use super::_entities::versions::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type Versions = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut this = self;
        let now = chrono::Utc::now().into();

        if insert {
            this.created_at = sea_orm::ActiveValue::Set(now);
            this.updated_at = sea_orm::ActiveValue::Set(now);
        } else if this.updated_at.is_unchanged() {
            this.updated_at = sea_orm::ActiveValue::Set(now);
        }

        Ok(this)
    }
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
