use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum ApiKeys {
    Table,
    Id,
    UserId,
    Key,
    Name,
    Permissions,
    ExpireOn,
    ExpireValue,
    UsageCount,
    CreatedAt,
    LastUsedAt,
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
                .table(ApiKeys::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ApiKeys::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(ApiKeys::UserId).big_integer().not_null())
                .col(
                    ColumnDef::new(ApiKeys::Key)
                        .string()
                        .not_null()
                        .unique_key(),
                )
                .col(ColumnDef::new(ApiKeys::Name).string().not_null())
                .col(ColumnDef::new(ApiKeys::Permissions).json().not_null())
                .col(ColumnDef::new(ApiKeys::ExpireOn).string().not_null())
                .col(ColumnDef::new(ApiKeys::ExpireValue).big_integer().null())
                .col(
                    ColumnDef::new(ApiKeys::UsageCount)
                        .big_integer()
                        .not_null()
                        .default(0),
                )
                .col(
                    ColumnDef::new(ApiKeys::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                )
                .col(
                    ColumnDef::new(ApiKeys::LastUsedAt)
                        .timestamp_with_time_zone()
                        .null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_api_keys_user")
                        .from(ApiKeys::Table, ApiKeys::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(ApiKeys::Table).to_owned())
            .await
    }
}
