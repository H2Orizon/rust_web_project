#[macro_use] extern crate rocket;

use controllers::comment_controller::post_coments;
use rocket::fs::{FileServer, relative};
use controllers::items_controller::{create, create_category, delete_item, get_item, get_items, item_edit, patch_item_edit, post_create, post_create_category};
use controllers::user_controller::{add_img, change_password, edit_profile, get_all_user, log_in, log_out, patch_change_password, patch_edit_profile, post_log_in, post_register, profile, register};
use controllers::home_controller::index;
use rocket::Config;
use rocket::fairing::AdHoc;
use rocket_dyn_templates::Template;
use crate::db::connect;

mod controllers;
mod services;
mod models;
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
            get_all_user
        ])
        .mount("/items/", routes![
            get_item, create, post_create,
            item_edit, patch_item_edit,
            create_category, post_create_category,
            delete_item, post_coments
        ])
        .mount("/user/", routes![
            edit_profile, patch_edit_profile
            ,change_password,patch_change_password
            ,add_img
        ])
        .mount("/static", FileServer::from("static"))
        .mount("/uploads", FileServer::from(relative!("uploads")))
        .attach(Template::fairing())
        .attach(AdHoc::on_ignite("Cookies Config", |rocket| async {
            rocket
        }))
}
