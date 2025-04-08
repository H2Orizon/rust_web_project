use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};
use thiserror::Error;

use crate::{models::{category_model::{ActiveModel as ActiveModelCategory, CategoryDTO, Entity as CategoryEntity, NewCategory}, 
        item_model::{ActiveModel as ActiveModelItem, Entity as ItemEntity, ItemDTO, NewItemForm}}, services::img_for_items_services::get_all_item_imgs};

#[derive(Debug, Error)]
pub enum ItemError {
    #[error("Failed to insert user into database")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Category not found")]
    CategoryNotFound,
    #[error("Item not found")]
    ItemNotFound
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

pub async fn create_category_f(db: &DatabaseConnection, form_data: &NewCategory) -> Result<(), ItemError> {
    let new_category = ActiveModelCategory{
        name: Set(form_data.name.clone()),
        ..Default::default()
    };
    println!("Нова катигорія: {:?}", new_category);
    match new_category.insert(db).await {
        Ok(_) => {
            println!("Катигорія успішно додана!");
            Ok(())
        }Err(e) => {
            eprintln!(" Помилка під час вставки катигорії: {:?}", e);
            Err(ItemError::DatabaseError(e))
        }
    }
}

pub async fn creat_new_item(db: &DatabaseConnection, form_data: &NewItemForm, user_id: i32) -> Result<(), ItemError> {
    let new_item = ActiveModelItem{
        name: Set(form_data.name.clone()),
        category_id: Set(form_data.category_id.clone()),
        price: Set(form_data.price.clone()),
        description: Set(form_data.description.clone()),
        user_id: Set(user_id),
        ..Default::default()
    };
    println!("Нова катигорія: {:?}", new_item);
    match new_item.insert(db).await {
        Ok(_) => {
            println!("Товар успішно додана!");
            Ok(())
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

pub async fn update_item(db: &DatabaseConnection, item_id: i32, form_data: &NewItemForm) -> Result<(), ItemError> {
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

    up_item.update(db).await.map_err(ItemError::DatabaseError)?;
    Ok(())
}

pub async fn delete_item_f(db: &DatabaseConnection, item_id: i32) -> Result<(), DbErr> {
    let _item = ItemEntity::delete_by_id(item_id).exec(db).await?;
    Ok(())
}