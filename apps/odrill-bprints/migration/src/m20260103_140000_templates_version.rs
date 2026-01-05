use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Templates {
    Table,
    Version,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Templates::Table)
                    .add_column(
                        ColumnDef::new(Templates::Version)
                            .string()
                            .not_null()
                            .default("0.0.0"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Templates::Table)
                    .drop_column(Templates::Version)
                    .to_owned(),
            )
            .await
    }
}
