#[macro_use] extern crate rocket;
use rocket::fs::{FileServer};
use rocket_dyn_templates::{Template,context};

mod controllers;

#[get("/<name>")]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _{
    rocket::build()
    .mount("/", routes![
            controllers::home::index,greet,
            controllers::items::get_items
        ])
    .mount("/static", FileServer::from("static"))
    .attach(Template::fairing())
}
