#[macro_use] extern crate rocket;
use controllers::user::{post_log_in, post_register};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod controllers;
mod services;

#[launch]
fn rocket() -> _{
    rocket::build()
    .mount("/", routes![
            controllers::home::index
            ,controllers::items::get_items
            ,controllers::user::log_in, post_log_in
            ,controllers::user::register, post_register
        ])
    .mount("/items/", routes![
        controllers::items::get_item
    ])
    .mount("/static", FileServer::from("static"))
    .attach(Template::fairing())
}
