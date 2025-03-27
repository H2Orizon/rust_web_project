use crate::models::user_model::{ActiveModel, NewUserForm};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use sea_orm::{Set, ActiveModelTrait, DatabaseConnection};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Failed to insert user into database")]
    DatabaseError(#[from] sea_orm::DbErr),
}

pub async fn creat_user(db: &DatabaseConnection, form_data: &NewUserForm) -> Result<(), UserError>{
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let password_hash = argon.hash_password(form_data.password.as_bytes(), &salt)
    .map_err(|_| UserError::DatabaseError(sea_orm::DbErr::Custom("Password hashing failed".to_string())))?
    .to_string();
    let new_user = ActiveModel{
        username: Set(form_data.username.clone()),
        email: Set(form_data.email.clone()),
        phone_num: Set(form_data.phone_num.clone()),
        password: Set(password_hash),
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