use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "packages",
            &[
                ("id", ColType::PkAuto),
                ("name", ColType::String),
                ("guid", ColType::Uuid),
                ("latest_version_hash", ColType::String),
                ("docs_url", ColType::StringNull),
                ("git_url", ColType::StringNull),
            ],
            &[("user", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "packages").await
    }
}
