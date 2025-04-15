use sea_orm::entity::prelude::*;
use rocket::form::FromForm;
use serde::Serialize;
use validator_derive::Validate;
use crate::models::item_model;
use crate::validators::{password_validator::validator_password, phone_validator::validate_phone_number};


// enum Roles {
//     ADMIN,
//     USER,
//     SELLER
// }

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_num: String,
    pub role: String,
    pub img_url: String
}

#[derive(FromForm, Validate)]
pub struct NewUserForm {
    #[validate(length(min=3, message="Юзернейм повинин бути не менше 3 символів"))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Пароль має бути не менше 8 символів"))]
    #[validate(custom(function = "validator_password"))]
    pub password: String,
    #[validate(custom(function = "validate_phone_number"))]
    pub phone_num: String,
    pub role: String,
}

#[derive(FromForm, Validate)]
pub struct EditUserForm {
    #[validate(length(min=3, message="Юзернейм повинин бути не менше 3 символів"))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "validate_phone_number"))]
    pub phone_num: String,
    pub role: String,
}

#[derive(FromForm, Validate)]
pub struct ChangePasswordForm {
    pub old_password: String,
    pub new_password: String,
    #[validate(length(min = 8, message = "Пароль має бути не менше 8 символів"))]
    #[validate(custom(function = "validator_password"))]
    pub new_password_confirm: String
}

#[derive(FromForm, Validate)]
pub struct LogInUserForm {
    #[validate(email)]
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct UserDTO {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub phone_num: String,
    pub role: String,
    pub img_url: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::models::item_model::Entity")]
    Item,
}

impl Related<item_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}