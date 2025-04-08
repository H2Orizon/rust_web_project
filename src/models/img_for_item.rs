use sea_orm::entity::prelude::*;
use serde::Serialize;
use crate::models::item_model;

#[derive(Clone,Debug,DeriveEntityModel, PartialEq)]
#[sea_orm(table_name="imgs_for_items")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub item_id: i32,
    pub img_url: String
}

#[derive(Serialize)]
pub struct ImgItemDTO{
    pub id: i32,
    pub img_url: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "item_model::Entity",
        from = "Column::ItemId",
        to = "item_model::Column::Id"
    )]
    Item,
}

impl Related<item_model::Entity> for Entity {
    fn to() -> RelationDef{
        Relation::Item.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}