use sea_orm::entity::prelude::*;
use serde::Serialize;
use validator_derive::Validate;
use crate::models::{category_model, user_model, comment_model};

use super::img_for_item::{self, ImgItemDTO};

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

#[derive(FromForm, Validate)]
pub struct NewItemForm{
    #[validate(length(min=3, message="Назва товару не менше 3 символа"))]
    pub name: String,
    pub category_id: i32,
    #[validate(range(min= 0.0))]
    pub price: f32,
    #[validate(length(min=10, message="Опис товару не менше 10 символа"))]
    pub description: String
}

#[derive(Serialize)]
pub struct ItemDTO{
    pub id: i32,
    pub name: String,
    pub category: String,
    pub price: f32,
    pub description: String,
    pub link_to: String,
    pub user_id: i32,
    pub imgs: Vec<ImgItemDTO>
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
    #[sea_orm(has_many = "crate::models::comment_model::Entity")]
    Comment,
    #[sea_orm(has_many = "crate::models::img_for_item::Entity")]
    ImgsForItems,
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
impl Related<comment_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}
impl Related<img_for_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ImgsForItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
