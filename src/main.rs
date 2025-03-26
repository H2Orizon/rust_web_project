#[macro_use] extern crate rocket;
use controllers::{items::{creat, item_edit, patch_item_edit, post_creat}, user::{log_out, post_log_in, post_register, profile, register}};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod controllers;
mod services;

#[launch]
fn rocket() -> _{
    rocket::build()
    .mount("/", routes![
            controllers::home::index
            ,controllers::items::get_items, creat
            ,controllers::user::log_in, post_log_in
            , register, post_register
            , log_out, profile
        ])
    .mount("/items/", routes![
        controllers::items::get_item
        , creat, post_creat
        , item_edit, patch_item_edit
    ])
    .mount("/static", FileServer::from("static"))
    .attach(Template::fairing())
}
