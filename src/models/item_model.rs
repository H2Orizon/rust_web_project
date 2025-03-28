use sea_orm::entity::prelude::*;
use rocket::form::FromForm;
use serde::Serialize;
use crate::models::category_model;

#[derive(Clone, Debug, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "items")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub category_id: i32,
    pub price: f32,
    pub description: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "category_model::Entity",
        from = "Column::CategoryId",
        to = "category_model::Column::Id"
    )]
    Category,
}

impl Related<category_model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
