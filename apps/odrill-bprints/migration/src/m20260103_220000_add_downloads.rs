use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Packages {
    Table,
    Downloads,
}

#[derive(Iden)]
enum Versions {
    Table,
    Downloads,
}

#[derive(Iden)]
enum PackageDownloads {
    Table,
    Id,
    PackageId,
    VersionId,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // Add downloads to packages
        m.alter_table(
            Table::alter()
                .table(Packages::Table)
                .add_column(
                    ColumnDef::new(Packages::Downloads)
                        .big_integer()
                        .not_null()
                        .default(0),
                )
                .to_owned(),
        )
        .await?;

        // Add downloads to versions
        m.alter_table(
            Table::alter()
                .table(Versions::Table)
                .add_column(
                    ColumnDef::new(Versions::Downloads)
                        .big_integer()
                        .not_null()
                        .default(0),
                )
                .to_owned(),
        )
        .await?;

        // Create package_downloads table for history
        m.create_table(
            Table::create()
                .table(PackageDownloads::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(PackageDownloads::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(PackageDownloads::PackageId)
                        .big_integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(PackageDownloads::VersionId)
                        .big_integer()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(PackageDownloads::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(PackageDownloads::Table).to_owned())
            .await?;

        m.alter_table(
            Table::alter()
                .table(Versions::Table)
                .drop_column(Versions::Downloads)
                .to_owned(),
        )
        .await?;

        m.alter_table(
            Table::alter()
                .table(Packages::Table)
                .drop_column(Packages::Downloads)
                .to_owned(),
        )
        .await
    }
}
