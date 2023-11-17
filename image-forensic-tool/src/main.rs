#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::fs::{relative, FileServer};
use rocket::response::content::RawHtml;
use std::env;
use tera::{Context, Tera};

#[get("/")]
fn index(tera: &rocket::State<Tera>) -> RawHtml<String> {
    dotenv().ok();

    let mut context = Context::new();
    context.insert(
        "google_maps_api_key",
        &env::var("GOOGLE_MAPS_API_KEY").expect("API key not found"),
    );

    RawHtml(
        tera.render("index.html", &context)
            .expect("Template rendering failed"),
    )
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .manage(Tera::new("static/*.html").expect("Error loading templates"))
}
