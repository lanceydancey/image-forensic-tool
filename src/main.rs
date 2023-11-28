#[macro_use]
extern crate rocket;
extern crate image_proc;

use dotenv::dotenv;
use rocket::fs::{relative, FileServer};
use rocket::response::content::RawHtml;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use tera::{Context, Tera};

#[derive(Deserialize, Serialize, Debug)]
struct ImageData {
    filename: String,
    date_created: Option<String>,
    gps_coordinates: Option<String>,
}

//https://blog.logrocket.com/rust-web-apps-using-rocket-framework/
#[get("/")]
fn index(tera: &rocket::State<Tera>) -> RawHtml<String> {
    dotenv().ok();
    let image_data: Vec<ImageData> = import_images().expect("Failed to load image data");

    let image_data_json =
        serde_json::to_string(&image_data).expect("Failed to serialize image data");

    println!("Serialized Image Data JSON: {}", &image_data_json);

    let mut context: Context = Context::new();
    context.insert(
        "google_maps_api_key",
        &env::var("GOOGLE_MAPS_API_KEY").expect("API key not found"),
    );
    context.insert("image_data", &image_data_json);

    RawHtml(
        tera.render("index.html", &context)
            .expect("Template rendering failed"),
    )
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    read_image();

    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .manage(Tera::new("static/*.html").expect("Error loading templates"))
}

fn import_images() -> Result<Vec<ImageData>, String> {
    let data = fs::read_to_string("output.json").map_err(|e| e.to_string())?;
    let images: Vec<ImageData> =
        serde_json::from_str(&data).map_err(|e: serde_json::Error| e.to_string())?;

    println!("Imported Images: {:?}", images);

    Ok(images)
}

fn read_image() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: program_name <path_to_folder>");
        std::process::exit(1);
    }

    let folder_path: &String = &args[1];
    println!("Processing folder: {}", folder_path);

    if let Err(e) = image_proc::process_images(folder_path) {
        eprintln!("Error processing images: {}", e);
    }
}
