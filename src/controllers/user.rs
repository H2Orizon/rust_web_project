use rocket_dyn_templates::{Template,context};
use rocket::response::Redirect;

#[get("/log_in")]
pub fn log_in() -> Template {
    Template::render("log_in", context! {title:"Log in"})
}

#[post("/log_in")]
pub fn post_log_in() -> Redirect{
    Redirect::to(uri!(register()))   
}

#[get("/register")]
pub fn register() -> Template {
    Template::render("register", context! {title:"Register"})
}

#[post("/register")]
pub fn post_register() -> Redirect{
    Redirect::to(uri!(log_in()))
}