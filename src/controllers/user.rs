use rocket_dyn_templates::{Template,context};
use rocket::http::{Cookie, CookieJar};
use rocket::State;
use rocket::{form::Form, response::Redirect};
use sea_orm::DatabaseConnection;
use crate::models::user_model::{ChangePasswordForm, EditUserForm, LogInUserForm};
use crate::services::user_service::{change_password_f, edit_profile_f, get_all_users, get_user_profile};
use crate::{models::user_model::NewUserForm, services::user_service::{create_user, log_in as log_inF}};

#[get("/log_in")]
pub fn log_in() -> Template {
    Template::render("user/log_in", context! {title:"Log in"})
}

#[post("/log_in", data = "<form_data>")]
pub async fn post_log_in(
    db: & State<DatabaseConnection>, 
    form_data: Form<LogInUserForm>,
    cookies: & CookieJar<'_>
) -> Redirect {
    match log_inF(&db, &form_data).await {
        Ok(user) => {
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            Redirect::to("/profile")
        },
        Err(_) => Redirect::to("/log_in?error=invalid_credentials"),
    }   
}

#[get("/register")]
pub fn register() -> Template {
    Template::render("user/register", context! {title:"Register"})
}

#[post("/register", data = "<form_data>")]
pub async fn post_register(
    db: &State<DatabaseConnection>, 
    form_data: Form<NewUserForm>
) -> Redirect {
    match create_user(db.inner(), &form_data).await {
        Ok(_) => Redirect::to("/log_in"),
        Err(_) => Redirect::to("/error"),
    }
}

#[get("/profile")]
pub async fn profile(db: &State<DatabaseConnection>, cookies: &CookieJar<'_>) -> Template{
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>(){
            match get_user_profile(db.inner(), user_id).await {
                Ok(user) =>{
                    Template::render("user/profile", context!{
                        title:"My profile"
                        ,username: user.username
                        ,email: user.email
                        ,phone_num: user.phone_num
                        ,role: user.role
                    })
                }
                Err(_) => Template::render("error/404", context! { message: "User not found" })
            }
        }else {
            Template::render("error/403", context! { message: "Invalid session" })
        }
    }else {
        Template::render("error/403", context! { message: "Invalid session" })
    }
}

#[post("/profile/log_out")]
pub fn log_out(cookies: &CookieJar<'_>) -> Redirect{
    cookies.remove_private("user_id");
    Redirect::to(uri!(crate::controllers::home::index))
}

#[get("/all_users")]
pub async fn get_all_user(db: &State<DatabaseConnection>) -> Template {
    let users = get_all_users(db).await.unwrap_or_default();
    Template::render("user/all_users", context!{
        title:"All users",
        message:"All users",
        users:users
    })
}

#[get("/edit_profile")]
pub fn edit_profile(cookies: &CookieJar<'_>) -> Template {
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>(){
            return Template::render("user/edit_profile", context!{title:"edit_profile"});
        }
    }
    Template::render("error/403", context! { message: "Invalid session" })
}

#[patch("/edit_profile", data = "<from_data>")]
pub async fn patch_edit_profile(db: &State<DatabaseConnection>, cookies: &CookieJar<'_>, from_data: Form<EditUserForm>) -> Redirect {
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>(){
            match edit_profile_f(db, user_id, &from_data).await {
                Ok(_) => return Redirect::to(uri!(profile)),
                Err(_) => return Redirect::to(uri!(edit_profile))
            }
        }
    }
    Redirect::to(uri!(log_in))
}

#[get("/change_password")]
pub fn change_password(cookies: & CookieJar<'_>) -> Template{
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>(){
            return Template::render("user/change_password", context!{title:"Зміна пароля"});
        }
    }
    Template::render("error/403", context! { message: "Invalid session" })
}

#[patch("/change_password", data = "<form_data>")]
pub async fn patch_change_password(db: &State<DatabaseConnection>, cookies: & CookieJar<'_>, form_data: Form<ChangePasswordForm>) -> Redirect {
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>(){
            match change_password_f(db, &form_data.into_inner(), user_id).await {
                Ok(_) => return Redirect::to(uri!(profile)),
                Err(_) => return Redirect::to(uri!(change_password))
            }
        }
    }
    Redirect::to(uri!(log_in))
}