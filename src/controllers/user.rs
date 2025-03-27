use rocket_dyn_templates::{Template,context};
use rocket::State;
use rocket::{form::Form, response::Redirect};
use sea_orm::DatabaseConnection;
use crate::{models::user_model::NewUserForm, services::user_service};

#[get("/log_in")]
pub fn log_in() -> Template {
    Template::render("user/log_in", context! {title:"Log in"})
}

#[post("/log_in")]
pub fn post_log_in() -> Redirect{
    Redirect::to(uri!(profile))   
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
    match user_service::creat_user(db.inner(), &form_data).await {
        Ok(_) => Redirect::to("/profile"),
        Err(_) => Redirect::to("/error"),
    }
}

#[get("/profile")]
pub fn profile() -> Template{
    Template::render("user/profile", context!{title:"Profile"})
}

#[post("/profile")]
pub fn log_out() -> Redirect{
    Redirect::to(uri!(crate::controllers::home::index))
}