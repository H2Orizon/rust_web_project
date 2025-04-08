use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use thiserror::Error;

use crate::models::img_for_item::{self, ActiveModel, Entity as ImgEntity, ImgItemDTO};

use super::help_service::delete_item_folder;

#[derive(Debug, Error)]
pub enum ImgError {
    #[error("Failed to insert user into database")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Img not found")]
    ImgNotFound,
}

pub async fn get_all_item_imgs(db: &DatabaseConnection, item_id: i32) -> Result<Vec<ImgItemDTO>, sea_orm::DbErr>{
    let imgs = ImgEntity::find()
    .filter(img_for_item::Column::ItemId.eq(item_id)).all(db).await?;
    let mut img_dtos = Vec::new();
    for img in imgs {
        img_dtos.push(ImgItemDTO{
            id: img.id,
            img_url: img.img_url
        });
    }
    Ok(img_dtos)
}

pub async fn get_item_img(db: &DatabaseConnection, img_id: i32) -> Result<ImgItemDTO, ImgError> {
    let img = ImgEntity::find_by_id(img_id)
    .one(db).await.map_err(|e| ImgError::DatabaseError(e))?
    .ok_or(ImgError::ImgNotFound)?;
    Ok(ImgItemDTO { id:img_id, img_url: img.img_url })
}

pub async fn add_img_to_item(db: &DatabaseConnection, item_id: i32, file: String) -> Result<(), String> {
    if !can_add_more_imgs(db, item_id).await.map_err(|e| e.to_string())?{
        return Err("Bruh".into());
    }
    let new_img= ActiveModel{
        item_id: Set(item_id),
        img_url: Set(file),
        ..Default::default()
    };
    new_img.insert(db).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_img_url_as_string(db: &DatabaseConnection, img_id: i32) -> String {
    get_item_img(db, img_id).await.map(|i| i.img_url).unwrap_or_else(|_| ImgError::ImgNotFound.to_string())
}

pub async fn can_add_more_imgs(db: &DatabaseConnection, item_id: i32) -> Result<bool, sea_orm::DbErr> {
    let count = ImgEntity::find()
    .filter(img_for_item::Column::ItemId.eq(item_id))
    .count(db)
    .await?;
    Ok(count < 5)
}

pub async fn delete_all_item_img(db: &DatabaseConnection, item_id: i32) -> Result<(), sea_orm::DbErr>{
    let img_url = item_id.to_string();
    if let Err(err) = delete_item_folder(&img_url).await{
        eprintln!("Помилка при видаленні деректорії: {:?}", err);
    }
    
    let _delete = ImgEntity::delete_many()
    .filter(img_for_item::Column::ItemId.eq(item_id))
    .exec(db).await?;
    Ok(())
}

pub async fn delete_image_db(db: &DatabaseConnection, img_id: i32) -> Result<(), sea_orm::DbErr>{
    let _img = ImgEntity::delete_by_id(img_id).exec(db).await?;
    Ok(())
}