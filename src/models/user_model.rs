use sea_orm::entity::prelude::*;
use rocket::form::FromForm;
use serde::Serialize;

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

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        unimplemented!()
    }
}

impl ActiveModelBehavior for ActiveModel {}