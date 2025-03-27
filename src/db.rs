use sea_orm::{Database, DatabaseConnection};
use std::env;
use dotenvy::dotenv;

pub async fn connect() -> Result<DatabaseConnection, sea_orm::DbErr> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL не встановлено");
    Database::connect(database_url).await
}
