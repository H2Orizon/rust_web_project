use rocket::response::Redirect;
use rocket_dyn_templates::{Template,context};

use crate::services::{self};

#[get("/items")]
pub fn get_items() -> Template {
    
    let items = services::product_service::get_all_item();
    Template::render("items/items", context! {title:"Items", products:items})
}

#[get("/<item_name>")]
pub fn get_item(item_name: &str) -> Template {
    Template::render("items/item", context! {title:item_name})
}

#[get("/creat")]
pub fn creat() -> Template {
    Template::render("items/add_item", context!{title:"Creat new item"})
}

#[post("/creat")]
pub fn post_creat() -> Redirect{
    Redirect::to(uri!(get_items))
}

#[get("/<item_name>/item_edit")]
pub fn item_edit(item_name: &str) ->Template {
    Template::render("items/edit_item", context!{title: format!("Edit Item {}", item_name)})
}

#[patch("/<item_name>/item_edit")]
pub fn patch_item_edit(item_name: &str) ->Template {
    Template::render("items/edit_item", context!{title: format!("Edit Item {}", item_name)})
}