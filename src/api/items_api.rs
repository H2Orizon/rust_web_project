use rocket::{form::Form, serde::json::Json, State};
use sea_orm::DatabaseConnection;
use rocket::http::Status;

use crate::{models::{category_model::{CategoryDTO, NewCategory}, img_for_item::ImgItemDTO, item_model::{ItemDTO, NewItemForm}}, 
services::{help_service::{delete_image, file_load_for_item, UploadForm}, img_for_items_services::{add_img_to_item, delete_image_db, get_img_url_as_string}, item_service::{create_category_f, create_new_item, delete_item_f, get_all_item, get_one_item, update_item}, session::UserSession}};

#[get("/items")]
pub async fn get_items_api(db: &State<DatabaseConnection>) -> Result<Json<Vec<ItemDTO>>, Json<String>> {
    match get_all_item(db).await {
        Ok(items) => {
            Ok(Json(items))
        }
        Err(e) => {
            Err(Json(format!("Помилка отримання товарів: {}", e)))
        }
    }
}

#[post("/create_category", data = "<form_data>")]
pub async fn create_category_api(db: &State<DatabaseConnection>, user_session: UserSession, form_data: Form<NewCategory>) -> Result<(Status, Json<CategoryDTO>), (Status, Json<String>)> {
    if user_session.user.role != "admin" {
        return Err((Status::Unauthorized, Json("Лише адміністратор може створювати категорії.".to_string())));
    }

    match create_category_f(db.inner(), &form_data).await {
        Ok(dto) => Ok((Status::Created, Json(dto))),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка створення категорії: {}", e)))),
    }
}

#[get("/<item_id>")]
pub async fn get_item_api(db: &State<DatabaseConnection>, item_id: i32) -> Result<Json<ItemDTO>, Json<String>> {
    match get_one_item(db, item_id).await {
        Ok(item) => {
                Ok(Json(item))
            }
        Err(e) => Err(Json(e.to_string())),
    }
}

#[post("/item_create", data="<form_data>")]
pub async fn item_create_api(db: &State<DatabaseConnection>, form_data: Form<NewItemForm>, user_session: UserSession) -> Result<(Status, Json<ItemDTO>), (Status, Json<String>)> {
    if user_session.user.id != 0{
        return Err((Status::Unauthorized, Json("Неавторизований користувач.".to_string())));
    }

    match create_new_item(db.inner(), &form_data, user_session.user.id).await {
        Ok(item) => Ok((Status::Created, Json(item))),
        Err(e) =>{ 
            Err((Status::BadRequest, Json(format!("Помилка створення товару: {}", e))))
        }
    }
}

#[patch("/<item_id>/item_edit", data = "<form_data>")]
pub async fn item_edit_api( db: &State<DatabaseConnection>, item_id: i32, form_data: Form<NewItemForm>, user_session: UserSession) -> Result<(Status, Json<ItemDTO>), (Status, Json<String>)> {
    if user_session.user.id == 0 {
        return Err((Status::Unauthorized, Json("Неавторизований користувач.".to_string())));
    }
    let item = match get_one_item(db, item_id).await {
        Ok(item) => item,
        Err(_) => {
            return Err((Status::NotFound, Json("Товар не знайено або ви не маєте до нього доступу".to_string())));
        }
    };

    if item.user_id != user_session.user.id {
        return Err((Status::Forbidden, Json("У вас немає доступу до редагування цього товару".to_string())));
    }

    match update_item(db, item_id, &form_data).await {
        Ok(dto) => Ok((Status::Ok, Json(dto))),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка при редагування товару: {}", e))))
    }
}

#[delete("/<item_id>/delete")]
pub async fn delete_item_api(db: &State<DatabaseConnection>, item_id: i32, user_session: UserSession) -> Result<(Status, Json<&'static str>), (Status, Json<String>)> {
    if user_session.user.id == 0{
        return Err((Status::Unauthorized, Json("Неавторизований користувач.".to_string())));
    }
    let item = match get_one_item(db, item_id).await {
        Ok(item) => item,
        Err(_) => {
            return Err((Status::NotFound, Json("Товар не знайено або ви не маєте до нього доступу".to_string())));
        }
    };
    if item.user_id != user_session.user.id {
        return Err((Status::Forbidden, Json("У вас немає доступу до редагування цього товару".to_string())));
    }

    match delete_item_f(db, item_id).await {
        Ok(_) => return Ok((Status::Ok, Json("Товар успішно видалино"))),
        Err(e) => return Err((Status::InternalServerError, Json(e.to_string())))
    }
}

#[post("/<item_id>/add_item_img", data = "<form_data>")]
pub async fn add_img_to_item_api<'r>(db: &State<DatabaseConnection>, item_id: i32, form_data: Form<UploadForm<'r>>, user_session: UserSession) -> Result<(Status, Json<ImgItemDTO>), (Status, Json<String>)> {
    if user_session.user.id == 0{
        return Err((Status::Unauthorized, Json("Неавторизований користувач.".to_string())));
    }
    let item = match get_one_item(db, item_id).await {
        Ok(item) => item,
        Err(_) => {
            return Err((Status::NotFound, Json("Товар не знайено або ви не маєте до нього доступу".to_string())));
        }
    };
    if item.user_id != user_session.user.id {
        return Err((Status::Forbidden, Json("У вас немає доступу до редагування цього товару".to_string())));
    }
    let filename = match file_load_for_item(&db,form_data, item_id).await {
        Ok(name) => name,
        Err(_) => {
            eprintln!("Не вдалося зберегти файл");
            return Err((Status::NotFound, Json("Товар не знайено або ви не маєте до нього доступу".to_string())));
        }
    };
    println!("Файл збережено в: {}", filename);

    match add_img_to_item(db, item_id, filename).await{
        Ok(dto) => Ok((Status::Ok, Json(dto))),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка при додавання картинки: {}", e))))
    }
}

#[delete("/<item_id>/<img_id>/delete")]
pub async fn delete_item_img_api(db: &State<DatabaseConnection>, img_id: i32, item_id: i32, user_session: UserSession) -> Result<(Status, Json<&'static str>), (Status, Json<String>)>  {
    if user_session.user.id == 0{
        return Err((Status::Unauthorized, Json("Неавторизований користувач.".to_string())));
    }
    let item = match get_one_item(db, item_id).await {
        Ok(item) => item,
        Err(_) => {
            return Err((Status::NotFound, Json("Товар не знайено або ви не маєте до нього доступу".to_string())));
        }
    };
    if item.user_id != user_session.user.id {
        return Err((Status::Forbidden, Json("У вас немає доступу до редагування цього товару".to_string())));
    }
    let img_url = get_img_url_as_string(db, img_id).await;
    match delete_image(&img_url).await {
        Ok(_) => {
            match delete_image_db(db, img_id).await{
                Ok(_) => return Ok((Status::Ok, Json("Картинка успішно видалино"))),
                Err(e) => return Err((Status::InternalServerError, Json(e.to_string())))
            }
            
        },
        Err(e) => return Err((Status::InternalServerError,Json( e.to_string())))
    }
}