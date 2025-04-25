use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use thiserror::Error;
use validator::Validate;

use crate::{models::{category_model::{ActiveModel as ActiveModelCategory, CategoryDTO, Entity as CategoryEntity, NewCategory}, item_model::{ActiveModel as ActiveModelItem, Entity as ItemEntity, ItemDTO, NewItemForm}}, services::img_for_items_services::get_all_item_imgs};

use super::{comment_service::delete_all_item_comments, img_for_items_services::delete_all_item_img};

#[derive(Debug, Error)]
pub enum ItemError {
    #[error("Failed to insert user into database")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Category not found")]
    CategoryNotFound,
    #[error("Item not found")]
    ItemNotFound,
    #[error("Validation failed: {0}")]
    ValidationError(validator::ValidationErrors),    
}

pub async fn get_all_item(db: &DatabaseConnection) -> Result<Vec<ItemDTO>, sea_orm::DbErr> {
    let items = ItemEntity::find().all(db).await?;
    let mut item_dtos = Vec::new();
    for itm in items {
        let img = get_all_item_imgs(db, itm.id).await?;
        let category = category_to_string(db,itm.category_id).await;
        println!("{}",category);
        item_dtos.push(ItemDTO {
            id: itm.id,
            name: itm.name,
            category,
            price: itm.price,
            description: itm.description,
            link_to: format!("/items/{}", itm.id),
            user_id: itm.user_id,
            imgs: img
        });
    }
    Ok(item_dtos)
}
pub async fn get_all_user_item(db: &DatabaseConnection, user_id: i32) -> Result<Vec<ItemDTO>, sea_orm::DbErr> {
    let items = ItemEntity::find().filter(<ItemEntity as sea_orm::EntityTrait>::Column::UserId.eq(user_id)).all(db).await?;
    let mut item_dtos = Vec::new();
    for itm in items {
        let img = get_all_item_imgs(db, itm.id).await?;
        let category = category_to_string(db,itm.category_id).await;
        println!("{}",category);
        item_dtos.push(ItemDTO {
            id: itm.id,
            name: itm.name,
            category,
            price: itm.price,
            description: itm.description,
            link_to: format!("/items/{}", itm.id),
            user_id: itm.user_id,
            imgs: img
        });
    }
    Ok(item_dtos)
}


async fn category_to_string(db: &DatabaseConnection, id: i32) -> String {
    get_category(db, id).await.map(|c| c.name).unwrap_or_else(|_| "Невідома категорія".to_string())
}

pub async fn get_one_item(db: &DatabaseConnection, item_id: i32) -> Result<ItemDTO, ItemError> {
    let item = ItemEntity::find_by_id(item_id).one(db)
    .await.map_err(|err| ItemError::DatabaseError(err))?
    .ok_or(ItemError::ItemNotFound)?;
    let category = category_to_string(db,item.category_id).await;
    let img = get_all_item_imgs(db, item.id).await?;
    Ok(ItemDTO { 
        id: item.id, 
        name: item.name, 
        category: category, 
        price: item.price, 
        description: item.description, 
        link_to: "".to_string(),
        user_id: item.user_id,
        imgs: img
    })
}

pub async fn create_category_f(db: &DatabaseConnection, form_data: &NewCategory) -> Result<CategoryDTO, ItemError> {
    form_data.validate().map_err(|e| ItemError::ValidationError(e))?;

    let new_category = ActiveModelCategory{
        name: Set(form_data.name.clone()),
        ..Default::default()
    };
    println!("Нова катигорія: {:?}", new_category);
    match new_category.insert(db).await {
        Ok(new_category) => {
            println!("Катигорія успішно додана!");
            Ok(
                CategoryDTO { 
                    id: new_category.id, 
                    name: new_category.name 
                }
            )
        }Err(e) => {
            eprintln!(" Помилка під час вставки катигорії: {:?}", e);
            Err(ItemError::DatabaseError(e))
        }
    }
}

pub async fn create_new_item(db: &DatabaseConnection, form_data: &NewItemForm, user_id: i32) -> Result<ItemDTO, ItemError> {
    form_data.validate().map_err(|e| ItemError::ValidationError(e))?;

    let new_item = ActiveModelItem{
        name: Set(form_data.name.clone()),
        category_id: Set(form_data.category_id.clone()),
        price: Set(form_data.price.clone()),
        description: Set(form_data.description.clone()),
        user_id: Set(user_id),
        ..Default::default()
    };
    println!("Нова ітем: {:?}", new_item);
    match new_item.insert(db).await {
        Ok(new_item) => {
            println!("Товар успішно додана!");
            Ok(ItemDTO { 
                id: new_item.id, 
                name: new_item.name, 
                category: category_to_string(db, new_item.id).await, 
                price: new_item.price, 
                description: new_item.description, 
                link_to: format!("/items/{}", new_item.id),
                user_id: new_item.user_id,
                imgs: get_all_item_imgs(db, new_item.id).await?,
            })
        }Err(e) => {
            eprintln!(" Помилка під час вставки товарв: {:?}", e);
            Err(ItemError::DatabaseError(e))
        }
    }
}

pub async fn get_all_categoris(db: &DatabaseConnection) -> Result<Vec<CategoryDTO>, sea_orm::DbErr>{
    let categories = CategoryEntity::find().all(db).await?;

    let category_dtos: Vec<CategoryDTO> = 
    categories.into_iter()
        .map(|cat| CategoryDTO{
            id: cat.id,
            name: cat.name
        }).collect();
    Ok(category_dtos)
}

pub async fn get_category(db: &DatabaseConnection, category_id: i32) -> Result<CategoryDTO, ItemError> {
    let category = CategoryEntity::find_by_id(category_id)
    .one(db).await.map_err(|err| ItemError::DatabaseError(err))?
    .ok_or(ItemError::CategoryNotFound)?;
    Ok(CategoryDTO { id: category.id, name: category.name })
}

pub async fn update_item(db: &DatabaseConnection, item_id: i32, form_data: &NewItemForm) -> Result<ItemDTO, ItemError> {
    let item = ItemEntity::find_by_id(item_id)
    .one(db)
    .await
    .map_err(ItemError::DatabaseError)?
    .ok_or(ItemError::ItemNotFound)?;
    let mut up_item: ActiveModelItem = item.into();
    up_item.name = Set(form_data.name.clone());
    up_item.category_id = Set(form_data.category_id);
    up_item.price = Set(form_data.price);
    up_item.description = Set(form_data.description.clone());

    let inserted_item = up_item.update(db).await.map_err(ItemError::DatabaseError)?;
    
    Ok(ItemDTO { 
        id: inserted_item.id, 
        name: inserted_item.name, 
        category: category_to_string(db, inserted_item.id).await, 
        price: inserted_item.price, 
        description: inserted_item.description, 
        link_to: format!("/items/{}", inserted_item.id),
        user_id: inserted_item.user_id, 
        imgs: get_all_item_imgs(db, inserted_item.id).await?, 
    })
}

pub async fn delete_item_f(db: &DatabaseConnection, item_id: i32) -> Result<(), DbErr> {
    if let Err(err) = delete_all_item_img(db, item_id).await{
        eprintln!("Помилка при видаленні картинок з бази данних: {:?}", err);
    }
    if let Err(err) = delete_all_item_comments(db, item_id).await{
        eprintln!("Помилка при видаленні коментарів з бази данних: {:?}", err);
    }
    let _item = ItemEntity::delete_by_id(item_id).exec(db).await?;
    Ok(())
}