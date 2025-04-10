use crate::models::{comment_model::{self, ActiveModel, CommentDTO, CommentForm, Entity as CommentEntity}, item_model::Entity as ItemEntity, user_model::Entity as UserEntity};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommentError {
    #[error("Failed to insert user into database")]
    DatabaseError(#[from] sea_orm::DbErr)
}

pub async fn get_all_item_comments(db: &DatabaseConnection, item_id: i32) -> Result<Vec<CommentDTO>, sea_orm::DbErr>{
    let comments = CommentEntity::find()
    .filter(<CommentEntity as sea_orm::EntityTrait>::Column::ItemId.eq(item_id))
    .all(db)
    .await?;
    let mut comment_dtos = Vec::new();

    for comment in comments {
    let user = comment.find_related(UserEntity).one(db).await?;
    let item = comment.find_related(ItemEntity).one(db).await?;
    comment_dtos.push(CommentDTO {
        id: comment.id,
        user_name: user.map(|u| u.username),
        item_name: item.map(|i| i.name),
        content: comment.content,
    });
    }
    Ok(comment_dtos)
}

pub async fn get_all_user_comments(db: &DatabaseConnection, user_id: i32) -> Result<Vec<CommentDTO>, sea_orm::DbErr>{
    let comments = CommentEntity::find()
    .filter(<CommentEntity as sea_orm::EntityTrait>::Column::UserId.eq(user_id))
    .all(db)
    .await?;
    let mut comment_dtos = Vec::new();

    for comment in comments {
    let user = comment.find_related(UserEntity).one(db).await?;
    let item = comment.find_related(ItemEntity).one(db).await?;
    comment_dtos.push(CommentDTO {
        id: comment.id,
        user_name: user.map(|u| u.username),
        item_name: item.map(|i| i.name),
        content: comment.content,
    });
    }
    Ok(comment_dtos)
}

// pub async fn get_item_comment(db: &DatabaseConnection, comment_id: i32) -> Result<CommentDTO, sea_orm::DbErr>{
//     let comment = CommentEntity::find_by_id(comment_id)
//     .one(db).await?    
//     .ok_or(sea_orm::DbErr::RecordNotFound(
//         format!("Comment {} not found", comment_id),
//     ))?;
//     Ok(CommentDTO { item_id: comment.item_id, user_id: comment.user_id, content: comment.content })
// }

pub async fn create_comment(db: &DatabaseConnection, user_id: i32, item_id: i32, form_data: &CommentForm) -> Result<(), CommentError> {
    let new_comment = ActiveModel{
        user_id: Set(user_id),
        item_id: Set(item_id),
        content: Set(form_data.content.clone()),
        ..Default::default()
    };
    match new_comment.insert(db).await {
        Ok(_) => {
            println!("Товар успішно додана!");
            Ok(())
        }Err(e) => {
            eprintln!(" Помилка під час вставки товарв: {:?}", e);
            Err(CommentError::DatabaseError(e))
        }
    }
}

pub async fn delete_comment_f(db: &DatabaseConnection, comment_id: i32) -> Result<(), sea_orm::DbErr> {
    let _delete = CommentEntity::delete_by_id(comment_id).exec(db).await?;
    Ok(())    
}

pub async fn delete_all_item_comments(db: &DatabaseConnection, item_id: i32) -> Result<(),sea_orm::DbErr>{
    let _delete = CommentEntity::delete_many()
    .filter(comment_model::Column::ItemId.eq(item_id)) 
    .exec(db).await?;
    Ok(())
}