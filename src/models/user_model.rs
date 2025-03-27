use sea_orm::entity::prelude::*;

#[derive(Clone,Debag,DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub phon_num: String,
    pub role: String
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl Related<super::user::Entity> for Relation {
    fn to() -> RelationDef {
        unimplemented!()
    }
}

impl ActiveModelBehavior for ActiveModel {}