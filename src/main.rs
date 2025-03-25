#[macro_use] extern crate rocket;
use rocket::fs::{FileServer};
use rocket_dyn_templates::{Template,context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {title:"Home page", message:"Test123"})
}

#[get("/<name>")]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _{
    rocket::build()
    .mount("/", routes![index])
    .mount("/static", FileServer::from("static"))
    .attach(Template::fairing())
}



// fn main() {
//     println!("Hello, world!");
// }
