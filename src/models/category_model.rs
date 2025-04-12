use sea_orm::entity::prelude::*;
use rocket::form::FromForm;
use serde::Serialize;
use crate::models::item_model;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "category")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String
}

#[derive(FromForm)]
pub struct NewCategory{
    pub name: String
}

#[derive(Serialize)]
pub struct CategoryDTO{
    pub id: i32,
    pub name: String
}

#[derive(FromForm)]
pub struct DeleteCommUrl {
    pub redirect_url: String,
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
