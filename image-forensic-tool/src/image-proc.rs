use serde::{Serialize, Deserialize};
//use clap::{Command, Arg};
use std::env;
//use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
struct ImageData {
    filename: String,
    date_created: Option<String>,
    gps_coordinates: Option<(f64, f64)>,
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: program_name <path_to_folder>");
        std::process::exit(1);
    }

    let folder_path = &args[1];

    println!("Folder path: {}", folder_path);

    /*let matches = Command::new("Image EXIF Reader")
        .version("1.0")
        .about("Reads EXIF data from images")
        .arg(Arg::new("path")
             .value_name("PATH")
             .help("Sets the path to the folder of images")
             .required(true)
             .index(1))
        .get_matches();
    */

    //let folder_path: String = matches.get_one::<T>("path").unwrap().to_string();

    //println!("Folder path: {}", folder_path);

    //process_images(Path::new(folder_path));
}

//fn process_images(path: &Path) {
   //read exif data, create struct objects, write jsons out to file
  //  }





