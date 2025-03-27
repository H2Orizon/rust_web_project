use crate::{db, models::user_model::{ActiveModel, NewUserForm}};
use sea_orm::{Set, ActiveModelTrait, DatabaseConnection};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Failed to insert user into database")]
    DatabaseError(#[from] sea_orm::DbErr),
}

pub async fn creat_user(db: &DatabaseConnection, form_data: &NewUserForm) -> Result<(), UserError>{
    let new_user = ActiveModel{
        username: Set(form_data.username.clone()),
        email: Set(form_data.email.clone()),
        phone_num: Set(form_data.phone_num.clone()),
        password: Set(form_data.password.clone()),
        role: Set(form_data.role.clone()),
        ..Default::default()
    };
    println!("Вставка користувача: {:?}", new_user);
    match new_user.insert(db).await {
        Ok(_) => {
            println!(" Користувач успішно доданий!");
            Ok(())
        }
        Err(e) => {
            eprintln!(" Помилка під час вставки користувача: {:?}", e);
            Err(UserError::DatabaseError(e))
        }
    }
}