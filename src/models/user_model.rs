use sea_orm::entity::prelude::*;
use rocket::form::FromForm;
use serde::Serialize;
use crate::models::item_model;


enum Roles {
    ADMIN,
    USER,
    SELLER
}

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_num: String,
    pub role: String
}

#[derive(FromForm)]
pub struct NewUserForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_num: String,
    pub role: String,
}

#[derive(FromForm)]
pub struct EditUserForm {
    pub username: String,
    pub email: String,
    pub phone_num: String,
    pub role: String,
}

#[derive(FromForm)]
pub struct ChangePasswordForm {
    pub old_password: String,
    pub new_password: String,
    pub new_password_confirm: String
}

#[derive(FromForm)]
pub struct LogInUserForm {
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub phone_num: String,
    pub role: String,
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