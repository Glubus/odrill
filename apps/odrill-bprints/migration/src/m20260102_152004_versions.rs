use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Versions {
    Table,
    Id,
    Hash,
    Version,
    Description,
    Url,
    Yanked,
    PackageId,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Packages {
    Table,
    Id,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Versions::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Versions::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Versions::Hash).string().not_null())
                .col(ColumnDef::new(Versions::Version).string().not_null())
                .col(ColumnDef::new(Versions::Description).text().null())
                .col(ColumnDef::new(Versions::Url).string().not_null())
                .col(
                    ColumnDef::new(Versions::Yanked)
                        .boolean()
                        .not_null()
                        .default(false),
                )
                .col(ColumnDef::new(Versions::PackageId).big_integer().not_null())
                .col(
                    ColumnDef::new(Versions::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .col(
                    ColumnDef::new(Versions::UpdatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_versions_package")
                        .from(Versions::Table, Versions::PackageId)
                        .to(Packages::Table, Packages::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Versions::Table).to_owned())
            .await
    }
}
