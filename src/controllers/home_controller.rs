use rocket_dyn_templates::{Template,context};
use crate::services::session::UserSession;

#[get("/")]
pub fn index(user_session: Option<UserSession>) -> Template {
    Template::render("index", context! {title:"Home page", message:"Test123", user:user_session.map(|s| s.user)})
}