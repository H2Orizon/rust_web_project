use crate::models::user_model::{ActiveModel, ChangePasswordForm, EditUserForm, Entity as User, LogInUserForm, Model as UserModel, NewUserForm, UserDTO};
use argon2::{ Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use sea_orm::{Set, ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use thiserror::Error;

use super::help_service::delete_image;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Failed to insert user into database")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("PasswordsDoNotMatch")]
    PasswordsDoNotMatch,
}

fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    if let Ok(parsed_hash) = PasswordHash::new(hash){
        argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
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

pub async fn change_password_f(db: &DatabaseConnection,form_data: &ChangePasswordForm, user_id: i32) -> Result<(), UserError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let user = get_user(db, user_id).await?;
    if !verify_password(&form_data.old_password, &user.password) {
        return Err(UserError::InvalidPassword);
    }
    if form_data.new_password != form_data.new_password_confirm {
        return Err(UserError::PasswordsDoNotMatch);
    }
    let password_hash = argon.hash_password(form_data.new_password.as_bytes(), &salt)
    .map_err(|_| UserError::DatabaseError(sea_orm::DbErr::Custom("Password hashing failed".to_string())))?
    .to_string();
    let mut user_edit: ActiveModel = user.into();
    user_edit.password = Set(password_hash);
    user_edit.update(db).await.map_err(UserError::DatabaseError)?;
    Ok(())
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

pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<UserDTO>, sea_orm::DbErr>{
    let users = User::find().all(db).await?;

    let user_dtos: Vec<UserDTO> = 
    users.into_iter()
    .map(|usr| UserDTO{
        username: usr.username,
        email: usr.email,
        phone_num: usr.phone_num,
        role: usr.role,
        img_url: usr.img_url
    }).collect();

    Ok(user_dtos)
}

pub async fn edit_profile_f(db: &DatabaseConnection, user_id: i32, form_data: &EditUserForm) -> Result<(), UserError>{
    let user = get_user(db, user_id).await?;
    let mut user_edit: ActiveModel = user.into();
    user_edit.email = Set(form_data.email.clone());
    user_edit.phone_num = Set(form_data.phone_num.clone());
    user_edit.username = Set(form_data.username.clone());
    user_edit.role = Set(form_data.role.clone());
    user_edit.update(db).await.map_err(UserError::DatabaseError)?;
    Ok(())
}

pub async fn get_user_profile(db: &DatabaseConnection, user_id: i32) -> Result<UserDTO, UserError> {
    let user = get_user(db, user_id).await?;
    Ok(UserDTO { username: user.username, 
        email: user.email, 
        phone_num: user.phone_num, 
        role: user.role,
        img_url: user.img_url
    })
}

pub async fn get_user(db: &DatabaseConnection, user_id: i32) -> Result<UserModel, UserError> {
    let user = User::find_by_id(user_id).one(db).await        
    .map_err(|err| UserError::DatabaseError(err))?
    .ok_or(UserError::UserNotFound)?;
    Ok(user)
}

pub async fn change_img(db: &DatabaseConnection, user_id: i32, file: String) -> Result<(), UserError>{
    let user = get_user(db, user_id).await?;
    if user.img_url != "default/default_user_img.png"{
        if let Err(e) = delete_image(&user.img_url).await {
            eprintln!("Помилка при видаленні зображення: {}", e);
        }    
    }
    let mut user_edit: ActiveModel = user.into();
    user_edit.img_url = Set(file);
    user_edit.update(db).await.map_err(UserError::DatabaseError)?;
    Ok(())
}