use rocket::{form::Form, http::CookieJar, request::FlashMessage, response::{Flash, Redirect}, State};
use rocket_dyn_templates::{Template,context};
use sea_orm::DatabaseConnection;

use crate::{models::{category_model::{DeleteCommUrl, NewCategory}, item_model::NewItemForm}, 
services::{comment_service::get_all_item_comments, help_service::{delete_image, file_load_for_item, UploadForm}, img_for_items_services::{add_img_to_item, delete_image_db, get_img_url_as_string}, item_service::{create_new_item, create_category_f, delete_item_f, get_all_categoris, get_all_item, get_one_item, update_item}}};

#[get("/items")]
pub async fn get_items(db: &State<DatabaseConnection>, flash: Option<FlashMessage<'_>>) -> Template {

    let (flash_msg, flash_kind) = if let Some(f) = &flash {
        (f.message(), f.kind())
    } else {
        ("", "")
    };

    let items = get_all_item(db).await.unwrap_or_default();
    Template::render("items/items", context! {
            title:"Items",
            items:items,                 
            flash_msg: flash_msg,
            flash_kind: flash_kind
        }
    )
}

#[get("/create_category")]
pub fn create_category(cookies: &CookieJar<'_>, flash: Option<FlashMessage<'_>>) -> Template{
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(_user_id) = user_id_cookie.value().parse::<i32>(){

            let (flash_msg, flash_kind) = if let Some(f) = &flash {
                (f.message(), f.kind())
            } else {
                ("", "")
            };

            Template::render("items/creat_new_category", context!{
                title:"Create new category",
                flash_msg: flash_msg,
                flash_kind: flash_kind
            });
        }
    }
    Template::render("error/403", context! { message: "Invalid session" })
}

#[post("/create_category", data = "<form_data>")]
pub async fn post_create_category(db: &State<DatabaseConnection>, cookies: &CookieJar<'_>, form_data: Form<NewCategory>) -> Flash<Redirect> {
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(_user_id) = user_id_cookie.value().parse::<i32>(){
            match create_category_f(db.inner(), &form_data).await{
                Ok(_) => Flash::success(Redirect::to("/items/create_category"), "Категорія успішно створена"),
                Err(e) => Flash::error(Redirect::to("/items/create_category"), &e.to_string())
            };
        }
    }
    Flash::error(Redirect::to("/log_in"), "Будь ласка, увійдіть у свій акаунт.")
}

#[get("/<item_id>")]
pub async fn get_item(db: &State<DatabaseConnection>, item_id: i32, cookies: &CookieJar<'_>) -> Template {
    let redirect_url = format!("/items/{}",item_id);
    let user_id = cookies.get_private("user_id")
        .and_then(|cookie| cookie.value().parse::<i32>().ok());
    match get_one_item(db,item_id).await{
        Ok(item) => {
            let comment_dtos = get_all_item_comments(db, item_id).await.unwrap_or_default();
            Template::render("items/item", context! {item: item, item_id:item_id, comments:comment_dtos, user_id:user_id, redirect_url:redirect_url})
        },
        Err(_) => Template::render("error/403", context! { message: "Invalid session" }),
    }
}

#[get("/item_create")]
pub async fn create(db: &State<DatabaseConnection>, cookies: &CookieJar<'_>, flash: Option<FlashMessage<'_>>) -> Template {
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(_user_id) = user_id_cookie.value().parse::<i32>(){

            let (flash_msg, flash_kind) = if let Some(f) = &flash {
                (f.message(), f.kind())
            } else {
                ("", "")
            };
            
            let categories = get_all_categoris(db).await.unwrap_or_default(); 
            return Template::render("items/add_item", context!{
                title:"Create new item", 
                categories:categories, 
                flash_msg: flash_msg, 
                flash_kind: flash_kind
        });
        }
    }
    Template::render("error/403", context! { message: "Invalid session" })
}

#[post("/item_create", data="<form_data>")]
pub async fn post_create(db: &State<DatabaseConnection>, form_data: Form<NewItemForm>, cookies: &CookieJar<'_>) -> Flash<Redirect> {
    if let Some(user_id_cookie) = cookies.get_private("user_id"){
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>(){
            match create_new_item(db.inner(), &form_data, user_id).await {
                Ok(_) => return Flash::success(Redirect::to(uri!(get_items)), "Товар створено успішно!"),
                Err(e) =>{ 
                    return Flash::error(Redirect::to("/items/item_create"), &e.to_string()) 
                }
            }
        }
    }
    Flash::error(Redirect::to("/log_in"), "Будь ласка, увійдіть у свій акаунт.")
}

#[get("/<item_id>/item_edit")]
pub async  fn item_edit( db: &State<DatabaseConnection>,item_id: i32, cookies: &CookieJar<'_>, flash: Option<FlashMessage<'_>>) ->Template {
    if let Some(user_id_cookie) = cookies.get_private("user_id") {
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>() {

            let (flash_msg, flash_kind) = if let Some(f) = &flash {
                (f.message(), f.kind())
            } else {
                ("", "")
            };

            match get_one_item(db, item_id).await {
                Ok(item) => {
                    if item.user_id == user_id {
                        let categories = get_all_categoris(db).await.unwrap_or_default(); 
                        return Template::render("items/edit_item", context!{
                                title: format!("Edit Item {}", item_id), 
                                categories:categories, 
                                item_id:item_id,
                                flash_msg: flash_msg, 
                                flash_kind: flash_kind
                            }
                        )
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
pub async fn patch_item_edit( db: &State<DatabaseConnection>, item_id: i32, form_data: Form<NewItemForm>) -> Flash<Redirect> {
    match update_item(db, item_id, &form_data).await {
        Ok(()) => {
            return Flash::success(Redirect::to(format!("/items/{}",item_id)), "Товар успіжно зміниний")
        }
        Err(e) => {
            return Flash::error(Redirect::to(format!("/items/{}",item_id)), &e.to_string());
        }
    }
}

#[delete("/<item_id>/delete", data="<form_data>")]
pub async fn delete_item(db: &State<DatabaseConnection>,item_id: i32, form_data: Form<DeleteCommUrl>) -> Flash<Redirect> {
    match delete_item_f(db, item_id).await {
        Ok(_) => return Flash::warning(Redirect::to(form_data.redirect_url.clone()), "Товар успішно видалино"),
        Err(e) => return Flash::error(Redirect::to(form_data.redirect_url.clone()), e.to_string()) 
    }
}

#[post("/<item_id>/add_item_img", data = "<form_data>")]
pub async fn post_add_img_to_item<'r>(db: &State<DatabaseConnection>, item_id: i32, form_data: Form<UploadForm<'r>>) -> Redirect {
    let filename = match file_load_for_item(&db,form_data, item_id).await {
        Ok(name) => name,
        Err(_) => {
            eprintln!("Не вдалося зберегти файл");
            return Redirect::to(format!("/items/{}",item_id))
        }
    };
    println!("Файл збережено в: {}", filename);

    if let Err(err) = add_img_to_item(db, item_id, filename).await{
        eprintln!("Помилка при оновленні картинки: {:?}", err);
    }

    Redirect::to(format!("/items/{}",item_id))
}

#[delete("/<item_id>/<img_id>/delete")]
pub async fn delete_item_img(db: &State<DatabaseConnection>, img_id: i32, item_id: i32) -> Redirect {
    let img_url = get_img_url_as_string(db, img_id).await;
    match delete_image(&img_url).await {
        Ok(_) => {
            match delete_image_db(db, img_id).await{
                Ok(_) => return Redirect::to(format!("/items/{}",item_id)),
                Err(_) => return Redirect::to(format!("/items/{}",item_id))
            }
            
        },
        Err(_) => return Redirect::to(format!("/items/{}",item_id))
    }
}