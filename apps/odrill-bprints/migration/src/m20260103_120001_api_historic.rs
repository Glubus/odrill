use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum ApiHistoric {
    Table,
    Id,
    KeyId,
    Action,
    PkgId,
    CreatedAt,
    IpAddress,
}

#[derive(Iden)]
enum ApiKeys {
    Table,
    Id,
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
                .table(ApiHistoric::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ApiHistoric::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(ApiHistoric::KeyId).big_integer().not_null())
                .col(ColumnDef::new(ApiHistoric::Action).string().not_null())
                .col(ColumnDef::new(ApiHistoric::PkgId).big_integer().null())
                .col(
                    ColumnDef::new(ApiHistoric::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .col(ColumnDef::new(ApiHistoric::IpAddress).string().null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_api_historic_key")
                        .from(ApiHistoric::Table, ApiHistoric::KeyId)
                        .to(ApiKeys::Table, ApiKeys::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_api_historic_pkg")
                        .from(ApiHistoric::Table, ApiHistoric::PkgId)
                        .to(Packages::Table, Packages::Id)
                        // Note: Using SetNull on generic integer might fail if column not nullable, but PkgId is nullable.
                        .on_delete(ForeignKeyAction::SetNull)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(ApiHistoric::Table).to_owned())
            .await
    }
}
