use rocket::{form::Form, http::CookieJar, response::Redirect, State};
use rocket_dyn_templates::{Template,context};
use sea_orm::DatabaseConnection;

use crate::{models::{category_model::NewCategory, item_model::NewItemForm}, 
services::product_service::{creat_new_item, create_category_f, get_all_categoris, get_all_item, get_one_item, update_item}};

#[get("/items")]
pub async fn get_items(db: &State<DatabaseConnection>) -> Template {
    let items = get_all_item(db).await.unwrap_or_default();
    Template::render("items/items", context! {title:"Items", products:items})
}

#[get("/create_category")]
pub fn create_category() -> Template{
    Template::render("items/creat_new_category", context!{title:"Create new category"})
}

#[post("/create_category", data = "<form_data>")]
pub async fn post_create_category(db: &State<DatabaseConnection>, form_data: Form<NewCategory>) -> Redirect{
    match create_category_f(db.inner(), &form_data).await {
        Ok(_) => Redirect::to("/items/create_category"),
        Err(_) => Redirect::to("/items/create_category")
    }
}

#[get("/<item_id>")]
pub async fn get_item(db: &State<DatabaseConnection>, item_id: i32) -> Template {
    match get_one_item(db,item_id).await{
        Ok(item) => Template::render("items/item", context! {item: item }),
        Err(_) => Template::render("error/403", context! { message: "Invalid session" }),
    }
}

#[get("/item_create")]
pub async fn create(db: &State<DatabaseConnection>) -> Template {
    let categories = get_all_categoris(db).await.unwrap_or_default(); 
    Template::render("items/add_item", context!{title:"Create new item", categories:categories})
    
}

#[post("/item_create", data="<form_data>")]
pub async fn post_create(db: &State<DatabaseConnection>, form_data: Form<NewItemForm>, cookies: &CookieJar<'_>) -> Redirect {
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>(){
            match creat_new_item(db.inner(), &form_data, user_id).await {
                Ok(_) => return Redirect::to("/items"),
                Err(_) => return Redirect::to("/items/item_create")
            }
        }
    }
    Redirect::to(uri!(get_items))
}

#[get("/<item_id>/item_edit")]
pub async  fn item_edit( db: &State<DatabaseConnection>,item_id: i32, cookies: &CookieJar<'_>) ->Template {
    if let Some(user_id_cookie) = cookies.get_private("user_id") {
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>() {
            match get_one_item(db, item_id).await {
                Ok(item) => {
                    if item.user_id == user_id {
                        let categories = get_all_categoris(db).await.unwrap_or_default(); 
                        return Template::render("items/edit_item", context!{title: format!("Edit Item {}", item_id), categories:categories, item_id:item_id})
                    }
                }
                Err(_) => {
                    return Template::render("error/403", context!{title: format!("Edit Item {}", item_id)})
                }
            }
        }
    }
    Template::render("error/403", context!{title: format!("Edit Item {}", item_id)})
}

#[patch("/<item_id>/item_edit", data = "<form_data>")]
pub async fn patch_item_edit( db: &State<DatabaseConnection>, item_id: i32, form_data: Form<NewItemForm>) -> Redirect {
    match update_item(db, item_id, &form_data).await {
        Ok(()) => {
            return Redirect::to(format!("/items/{}",item_id));
        }
        Err(_) => {
            return Redirect::to(uri!(get_item(item_id)));
        }
    }
}