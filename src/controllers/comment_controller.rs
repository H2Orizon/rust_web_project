use rocket::form::Form;
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::DatabaseConnection;
use crate::models::category_model::DeleteCommUrl;
use crate::models::comment_model::CommentForm;

use crate::services::comment_service::{create_comment, delete_comment_f, updata_comment};

#[post("/<item_id>/post_comment", data="<form_data>")]
pub async fn post_coments(db: &State<DatabaseConnection>, item_id: i32, form_data: Form<CommentForm>, cookies: &CookieJar<'_>) -> Redirect {
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>(){
            match create_comment(db, user_id, item_id, &form_data).await {
                Ok(_) => return Redirect::to(format!("/items/{}",item_id)),
                Err(_) => return Redirect::to(format!("/items/{}",item_id))
            }
        }
    }
    Redirect::to("/log_in")
}

#[delete("/<item_id>/<comment_id>/delete_comment", data = "<form_data>")]
pub async fn delete_comment(db: &State<DatabaseConnection> ,form_data: Form<DeleteCommUrl>, item_id: i32, comment_id: i32, cookies: &CookieJar<'_>) ->Redirect {
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(_user_id) = user_id_cookie.value().parse::<i32>(){
            println!("{}",form_data.redirect_url.clone());
            match delete_comment_f(&db,comment_id).await {
                Ok(_) => return Redirect::to(form_data.redirect_url.clone()),
                Err(_) => return Redirect::to(form_data.redirect_url.clone())
            }
        }
    }
    Redirect::to("/log_in")
}

#[patch("/<item_id>/<comment_id>/edit_comment", data = "<form_data>")]
pub async fn edit_comment(db: &State<DatabaseConnection>, form_data: Json<CommentForm>, item_id: i32, comment_id: i32) -> Result<Json<CommentForm>, Status>{
    updata_comment(db, comment_id, &form_data)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(form_data.into_inner()))
}