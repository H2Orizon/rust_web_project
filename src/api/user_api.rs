use rocket::serde::json::Json;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::State;
use rocket::form::Form;
use sea_orm::DatabaseConnection;
use crate::models::user_model::{ChangePasswordForm, EditUserForm, LogInUserForm, UserDTO};
use crate::services::session::UserSession;
use crate::services::user_service::{change_img, change_password_f, edit_profile_f, get_all_users, get_user_dto};
use crate::{models::user_model::NewUserForm, services::user_service::{create_user, log_in as log_inF}};
use crate::services::help_service::{file_load,UploadForm};

#[post("/log_in", data = "<form_data>")]
pub async fn log_in_api(
    db: & State<DatabaseConnection>, 
    form_data: Form<LogInUserForm>,
    cookies: & CookieJar<'_>
) -> Result<(Status, Json<UserDTO>), (Status, Json<String>)>{
    match log_inF(&db, &form_data).await {
        Ok(user) => {
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            let dto = UserDTO{
                id: user.id,
                username: user.username,
                email: user.email,
                phone_num: user.phone_num,
                role: user.role,
                img_url: user.img_url
            };
            Ok((Status::Ok,Json(dto)))
        },
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка логіну: {}", e))))
    }   
}

#[post("/register", data = "<form_data>")]
pub async fn register_api(
    db: &State<DatabaseConnection>, 
    form_data: Form<NewUserForm>
) -> Result<(Status, Json<UserDTO>), (Status, Json<String>)> {
    match create_user(db.inner(), &form_data).await {
        Ok(dto) => Ok((Status::Ok,Json(dto))),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка регістрації: {}", e))))
    }
}

#[get("/<user_id>/profile")]
pub async fn get_user_api(db: &State<DatabaseConnection>, user_id: i32) -> Result<(Status, Json<UserDTO>), (Status, Json<String>)>{
    match get_user_dto(db.inner(), user_id).await {
        Ok(user) =>Ok((Status::Ok,Json(user))),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка отримання користувача: {}", e))))
    }
} 

#[post("/api/log_out")]
pub fn log_out_api(cookies: &CookieJar<'_>) -> Json<&'static str> {
    cookies.remove_private("user_id");
    Json("Logged out successfully")
}

#[get("/all_users")]
pub async fn get_all_user_api(db: &State<DatabaseConnection>) -> Result<(Status, Json<Vec<UserDTO>>), (Status, Json<String>)> {
    match get_all_users(db).await{
        Ok(users) => Ok((Status::Ok,Json(users))),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка отримання користувачів: {}", e))))
    }
}

#[patch("/edit_profile", data = "<form_data>")]
pub async fn edit_profile_api(db: &State<DatabaseConnection>,user_session: UserSession, form_data: Form<EditUserForm>) -> Result<(Status, Json<UserDTO>), (Status, Json<String>)> {
    if user_session.user.id != 0{
        return Err((Status::Unauthorized, Json("Неавторизований користувач.".to_string())));
    }
    match edit_profile_f(db, user_session.user.id, &form_data).await {
        Ok(dto) => return Ok((Status::Ok,Json(dto))),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка при редагування профілю: {}", e))))

    }
}

#[post("/add_img", data = "<form_data>")]
pub async fn add_img_api<'r>(db: &State<DatabaseConnection>,form_data: Form<UploadForm<'r>>, user_session: UserSession) -> Result<Json<&'static str>, (Status, Json<String>)>{
    let dir = "user_img";

    if user_session.user.id == 0{
        return Err((Status::Unauthorized, Json("Неавторизований користувач.".to_string())));
    }
    
    let filename = match file_load(form_data, &dir).await {
        Ok(name) => name,
        Err(_) => {
            eprintln!("Не вдалося зберегти файл");
            return Err((Status::NotFound, Json("Не вдалося зберегти файл".to_string())));
        }
    };

    println!("Файл збережено в: {}", filename);

    match change_img(db, user_session.user.id, filename).await {
        Ok(_) => Ok(Json("Картинка успішно змінина успішно змінено")),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка при додавання картинки: {}", e))))
    }
}

#[patch("/change_password", data = "<form_data>")]
pub async fn change_password_api(db: &State<DatabaseConnection>, user_session: UserSession, form_data: Form<ChangePasswordForm>) -> Result<Json<&'static str>, (Status, Json<String>)> {
    if user_session.user.id != 0{
        return Err((Status::Unauthorized, Json("Неавторизований користувач.".to_string())));
    }
    match change_password_f(db, &form_data.into_inner(), user_session.user.id).await {
        Ok(_) => Ok(Json("Пароль успішно змінено")),
        Err(e) => Err((Status::BadRequest, Json(format!("Помилка при зміні паролю: {}", e))))

    }
}