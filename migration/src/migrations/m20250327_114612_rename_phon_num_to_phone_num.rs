use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(User::Table)
                .rename_column(User::PhonNum, User::PhoneNum)
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(User::Table)
                .rename_column(User::PhoneNum, User::PhonNum)
                .to_owned()
        ).await
    }
}

#[derive(Iden)]
enum User {
    Table,
    PhonNum,
    PhoneNum,
}
