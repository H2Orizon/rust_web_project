use crate::models::user_model::{ActiveModel, LogInUserForm, NewUserForm, Entity as User, Model as UserModel, UserDTO};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, PasswordHash};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use sea_orm::{Set, ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Failed to insert user into database")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid password")]
    InvalidPassword,
}

pub async fn create_user(db: &DatabaseConnection, form_data: &NewUserForm) -> Result<(), UserError>{
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

pub async fn log_in(db: &DatabaseConnection, form_data: &LogInUserForm) -> Result<UserModel, UserError> {
    let user = User::find()
        .filter(<User as EntityTrait>::Column::Email.eq(form_data.email.clone()))
        .one(db).await
        .map_err(|err| UserError::DatabaseError(err))?;
    if let Some(user) = user {
        if verify_password(&form_data.password, &user.password) {
            Ok(user)
        } else {
            Err(UserError::InvalidPassword)
        }
    } else {
        Err(UserError::UserNotFound)
    }
}

fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    if let Ok(parsed_hash) = PasswordHash::new(hash){
        argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
}

pub async fn get_user_profile(db: &DatabaseConnection, user_id: i32) -> Result<UserDTO, UserError> {
    let user = User::find_by_id(user_id).one(db).await        
    .map_err(|err| UserError::DatabaseError(err))?
    .ok_or(UserError::UserNotFound)?;

    Ok(UserDTO { username: user.username, 
        email: user.email, 
        phone_num: user.phone_num, 
        role: user.role 
    })
}