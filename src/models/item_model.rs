use sea_orm::entity::prelude::*;
// use rocket::form::FromForm;
use serde::Serialize;
use crate::models::{category_model, user_model};

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "item")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub category_id: i32,
    pub price: f32,
    pub description: String,
    pub user_id: i32
}

#[derive(FromForm)]
pub struct NewItemForm{
    pub name: String,
    pub category_id: i32,
    pub price: f32,
    pub description: String
}

#[derive(Serialize)]
pub struct ItemDTO{
    pub id: i32,
    pub name: String,
    pub category: String,
    pub price: f32,
    pub description: String,
    pub link_to: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "category_model::Entity",
        from = "Column::CategoryId",
        to = "category_model::Column::Id"
    )]
    Category,
    #[sea_orm(
        belongs_to = "user_model::Entity",
        from = "Column::UserId",
        to = "user_model::Column::Id"
    )]
    User,
}

impl Related<category_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}
impl Related<user_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
