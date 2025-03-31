use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Таблиця Category
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Таблиця User
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key()) // email має бути унікальним
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::PhoneNum).string().not_null())
                    .col(ColumnDef::new(User::Role).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Таблиця Item
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Item::Name).string().not_null())
                    .col(ColumnDef::new(Item::CategoryId).integer().not_null())
                    .col(ColumnDef::new(Item::UserId).integer().not_null()) // Додаємо user_id
                    .col(ColumnDef::new(Item::Price).decimal().not_null())
                    .col(ColumnDef::new(Item::Description).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-item-category")
                            .from(Item::Table, Item::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-item-user")
                            .from(Item::Table, Item::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Item::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Category::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Email,
    Password,
    PhoneNum,
    Role,
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id,
    Name,
    CategoryId,
    UserId, // Додаємо UserId
    Price,
    Description,
}
