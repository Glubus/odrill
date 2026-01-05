use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Packages {
    Table,
    Id,
    Name,
    Guid,
    LatestVersionHash,
    DocsUrl,
    GitUrl,
    UserId,
    CreatedAt,
    UpdatedAt,
}
#[derive(Iden)]
enum Users {
    Table,
    Id,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Packages::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Packages::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Packages::Name).string().not_null())
                .col(ColumnDef::new(Packages::Guid).uuid().not_null())
                .col(
                    ColumnDef::new(Packages::LatestVersionHash)
                        .string()
                        .not_null(),
                )
                .col(ColumnDef::new(Packages::DocsUrl).string().null())
                .col(ColumnDef::new(Packages::GitUrl).string().null())
                .col(ColumnDef::new(Packages::UserId).big_integer().not_null())
                .col(
                    ColumnDef::new(Packages::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .col(
                    ColumnDef::new(Packages::UpdatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_packages_user")
                        .from(Packages::Table, Packages::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Packages::Table).to_owned())
            .await
    }
}
