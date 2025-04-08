use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ImgsForItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ImgsForItems::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ImgsForItems::ItemId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ImgsForItems::ImgUrl)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ImgsForItems::Table, ImgsForItems::ItemId)
                            .to(Item::Table, Item::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ImgsForItems::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ImgsForItems {
    Table,
    Id,
    ItemId,
    ImgUrl,
}

#[derive(Iden)]
enum Item {
    Table,
    Id,
}