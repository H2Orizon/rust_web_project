use rocket_dyn_templates::{Template,context};


#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {title:"Home page", message:"Test123"})
}