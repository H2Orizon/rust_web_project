use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Comment::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Comment::UserId).integer().not_null())
                    .col(ColumnDef::new(Comment::ItemId).integer().not_null())
                    .col(ColumnDef::new(Comment::Content).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comment::Table, Comment::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Comment::Table, Comment::ItemId)
                            .to(Item::Table, Item::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Comment::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum Comment {
    Table,
    Id,
    UserId,
    ItemId,
    Content,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}

#[derive(Iden)]
enum Item {
    Table,
    Id,
}