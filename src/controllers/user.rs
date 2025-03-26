use rocket_dyn_templates::{Template,context};
use rocket::response::Redirect;

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

#[post("/register")]
pub fn post_register() -> Redirect{
    Redirect::to(uri!(log_in))
}

#[get("/profile")]
pub fn profile() -> Template{
    Template::render("user/profile", context!{title:"Profile"})
}

#[post("/profile")]
pub fn log_out() -> Redirect{
    Redirect::to(uri!(crate::controllers::home::index))
}