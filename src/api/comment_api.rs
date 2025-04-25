use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::DatabaseConnection;
use crate::models::category_model::DeleteCommUrl;
use crate::models::comment_model::{CommentDTO, CommentForm};
use crate::services::comment_service::{create_comment, delete_comment_f, updata_comment};
use crate::services::session::UserSession;


#[post("/<item_id>/post_comment", data="<form_data>")]
pub async fn post_coments_api(db: &State<DatabaseConnection>, item_id: i32, form_data: Form<CommentForm>, user_session: UserSession) -> Result<(Status, Json<CommentDTO>), (Status, Json<String>)> {
    if user_session.user.id == 0 {
        return Err((Status::Unauthorized, Json("Неавторизований користувач.".to_string())));
    }
    match create_comment(db, user_session.user.id, item_id, &form_data).await {
        Ok(dto) => Ok((Status::Ok, Json(dto))),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка при свторення коментаря: {}", e))))
    }
}

#[delete("/<comment_id>/delete_comment", data = "<form_data>")]
pub async fn delete_comment_api(db: &State<DatabaseConnection> ,form_data: Form<DeleteCommUrl>, comment_id: i32, user_session: UserSession) -> Result<(Status, Json<&'static str>), (Status, Json<String>)> {
    if user_session.user.id != 0{
        return Err((Status::Unauthorized, Json("Неавторизований користувач.".to_string())));
    }
    println!("{}",form_data.redirect_url.clone());
    match delete_comment_f(&db,comment_id).await {
        Ok(_) => return Ok((Status::Ok, Json("Коментар успішно видалино"))),
        Err(e) => return Err((Status::InternalServerError, Json(e.to_string())))
    }
}

#[patch("/<comment_id>/edit_comment", data = "<form_data>")]
pub async fn edit_comment_api(db: &State<DatabaseConnection>, form_data: Json<CommentForm>, comment_id: i32) -> Result<(Status, Json<CommentDTO>), (Status, Json<String>)>{
    match updata_comment(db, comment_id, &form_data).await{
        Ok(dto) => Ok((Status::Ok, Json(dto))),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка при редагування коментаря: {}", e))))
    }
}