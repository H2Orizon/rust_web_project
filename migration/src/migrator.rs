use sea_orm_migration::prelude::*;

use crate::migrations;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migrations::m20220101_000001_create_table::Migration),
            Box::new(migrations::m20250326_191821_create_users_table::Migration),
            Box::new(migrations::m20250327_114612_rename_phon_num_to_phone_num::Migration),
            Box::new(migrations::m20250327_120459_add_password_to_user::Migration),
            Box::new(migrations::m20250328_124122_create_categories_table_and_item_table::Migration),
        ]
    }
}
