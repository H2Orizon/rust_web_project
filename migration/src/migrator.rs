use sea_orm_migration::prelude::*;

use crate::migrations;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migrations::m20220101_000001_create_table::Migration),
            Box::new(migrations::m20250326_191821_create_users_table::Migration),
        ]
    }
}
