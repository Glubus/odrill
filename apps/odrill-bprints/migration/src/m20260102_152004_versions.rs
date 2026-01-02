use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "versions",
            &[
                ("id", ColType::PkAuto),
                ("hash", ColType::String),
                ("version", ColType::String),
                ("description", ColType::TextNull),
                ("url", ColType::String),
                ("yanked", ColType::Boolean),
            ],
            &[("package", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "versions").await
    }
}
