use rocket_dyn_templates::{Template,context};


#[get("/items")]
pub fn get_items() -> Template {
    let items = vec!["Ноутбук", "Смартфон", "Навушники"];

    Template::render("items", context! {title:"Items", products:items})
}