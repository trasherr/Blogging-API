use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Uuid).uuid().unique_key().not_null())
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .col(ColumnDef::new(Post::Image).string().not_null())
                    .col(ColumnDef::new(Post::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Post::UserId).integer().not_null())
                    .foreign_key(ForeignKey::create().name("fk-posts-users-id").from(Post::Table,Post::UserId).to(User::Table, User::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Post {
    Table,
    Id,
    Title,
    Uuid,
    Text,
    Image,
    UserId,
    CreatedAt
}
