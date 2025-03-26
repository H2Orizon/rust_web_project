use rocket_dyn_templates::{Template,context};

use crate::services::{self, product_service};

#[get("/items")]
pub fn get_items() -> Template {
    
    let items = services::product_service::get_all_item();
    Template::render("items", context! {title:"Items", products:items})
}

#[get("/<item_name>")]
pub fn get_item(item_name: &str) -> Template {
    Template::render("item", context! {title:item_name, })
}