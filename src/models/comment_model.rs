use sea_orm::entity::prelude::*;
use serde::Serialize;
use crate::models::{item_model, user_model};

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "comment")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub item_id: i32,
    pub content: String
}

#[derive(FromForm)]
pub struct CommentForm{
    pub content: String
}

#[derive(Serialize)]
pub struct CommentDTO{
    pub item_id: i32,
    pub user_id: i32,
    pub content: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user_model::Entity",
        from = "Column::UserId",
        to = "user_model::Column::Id"
    )]
    User,

    #[sea_orm(
        belongs_to = "item_model::Entity",
        from = "Column::ItemId",
        to = "item_model::Column::Id"
    )]
    Item,
}

impl Related<item_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}