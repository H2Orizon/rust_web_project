use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
        .create_table(
            Table::create()
                .table(Post::Table)
                .col(ColumnDef::new(Post::Id).integer().not_null().primary_key())
                .to_owned(),
        )
        .await?;
    Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
