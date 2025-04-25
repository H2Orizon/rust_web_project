#[macro_use] extern crate rocket;

use api::comment_api::{delete_comment_api, edit_comment_api, post_coments_api};
use api::items_api::{add_img_to_item_api, create_category_api, delete_item_api, delete_item_img_api, get_item_api, get_items_api, item_create_api, item_edit_api};
use api::user_api::{add_img_api, change_password_api, edit_profile_api, get_all_user_api, get_user_api, log_in_api, log_out_api, register_api};
use controllers::comment_controller::{delete_comment, edit_comment, post_coments};
use rocket::fs::{FileServer, relative};
use controllers::items_controller::{create, create_category, delete_item, delete_item_img, get_item, get_items, item_edit, patch_item_edit, post_add_img_to_item, post_create, post_create_category};
use controllers::user_controller::{add_img, change_password, edit_profile, get_all_user, log_in, log_out, patch_change_password, patch_edit_profile, post_log_in, post_register, profile, profile_user, register};
use controllers::home_controller::index;
use rocket::Config;
use rocket::fairing::AdHoc;
use rocket_dyn_templates::Template;
use crate::db::connect;

mod controllers;
mod services;
mod models;
mod validators;
mod api;
mod db;

#[launch]
async fn rocket() -> _ {
    let db = connect().await.expect("Помилка підключення до БД");
    let secret_key = "e6a1f4d2c7b8e9d3f5a6b4c2e8d9f3a7c1b5e4d2f9a3c6b7e1d8f2c4a9b5e7c3";
    let figment = Config::figment().merge(("secret_key", secret_key));

    rocket::custom(figment)
        .manage(db)
        .mount("/", routes![
            index, get_items,
            log_in, post_log_in,
            register, post_register,
            log_out, profile,
            get_all_user, profile_user
        ])
        .mount("/items/", routes![
            get_item, create, post_create,
            item_edit, patch_item_edit,
            create_category, post_create_category,
            delete_item, post_coments,
            post_add_img_to_item, delete_item_img,
            delete_comment, edit_comment
        ])
        .mount("/user/", routes![
            edit_profile, patch_edit_profile
            ,change_password,patch_change_password
            ,add_img
        ])
        .mount("api/items/",routes![
            get_item_api, get_items_api
            ,delete_item_api, delete_item_img_api
            ,create_category_api, item_create_api
            ,item_edit_api, add_img_to_item_api
            ,post_coments_api, delete_comment_api
            ,edit_comment_api
        ])
        .mount("api/user/", routes![
            log_in_api,register_api
            ,change_password_api,get_user_api
            ,get_all_user_api,log_out_api
            ,edit_profile_api, add_img_api
        ])
        .mount("/static", FileServer::from("static"))
        .mount("/uploads", FileServer::from(relative!("uploads")))
        .attach(Template::fairing())
        .attach(AdHoc::on_ignite("Cookies Config", |rocket| async {
            rocket
        }))
}
