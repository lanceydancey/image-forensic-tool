use serde::{Deserialize, Serialize};
//use clap::{Command, Arg};
use exif::{Exif, In, Reader, Tag, Value};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

    if let Err(e) = read_exif_data(folder_path) {
        eprintln!("Error reading EXIF data: {}", e);
    }

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
fn read_exif_data<P: AsRef<Path>>(file_path: P) -> Result<Exif, String> {
    //OPENER
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to open file: {}", e)),
    };

    //READER
    let mut buf = Vec::new();
    if let Err(e) = file.read_to_end(&mut buf) {
        return Err(format!("Failed to read file: {}", e));
    }

    //PARSER
    let exif = match Reader::new().read_from_container(&mut std::io::Cursor::new(&buf)) {
        Ok(exif) => exif,
        Err(e) => return Err(format!("Failed to read EXIF data: {}", e)),
    };

    println!("Successfully read EXIF data from the file.");

    //DATETIME
    if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        match field.value {
            Value::Ascii(ref vec) if !vec.is_empty() => {
                if let Ok(datetime) = std::str::from_utf8(&vec[0]) {
                    println!("DateTimeOriginal: {}", datetime);
                }
            }
            _ => println!("DateTimeOriginal tag found, but couldn't read the value."),
        }
    } else {
        println!("DateTimeOriginal tag is missing.");
    }
    //LATITUDE
    let latitude = if let Some(field) = exif.get_field(Tag::GPSLatitude, In::PRIMARY) {
        if let Value::Rational(lat_values) = &field.value {
            Some(lat_values.clone())
        } else {
            None
        }
    } else {
        None
    };

    let lat_ref = if let Some(field) = exif.get_field(Tag::GPSLatitudeRef, In::PRIMARY) {
        if let Value::Ascii(ref vec) = field.value {
            std::str::from_utf8(&vec[0]).ok()
        } else {
            None
        }
    } else {
        None
    };

    if let (Some(lat), Some(lat_ref)) = (latitude, lat_ref) {
        if let Some(lat_formatted) = format_gps_data(&lat, lat_ref) {
            println!("Latitude: {}", lat_formatted);
        }
    } else {
        println!("GPSLatitude or GPSLatitudeRef tag is missing.");
    }
    //LONGITUDE
    let longitude = if let Some(field) = exif.get_field(Tag::GPSLongitude, In::PRIMARY) {
        if let Value::Rational(lon_values) = &field.value {
            Some(lon_values.clone())
        } else {
            None
        }
    } else {
        None
    };

    let lon_ref = if let Some(field) = exif.get_field(Tag::GPSLongitudeRef, In::PRIMARY) {
        if let Value::Ascii(ref vec) = field.value {
            std::str::from_utf8(&vec[0]).ok()
        } else {
            None
        }
    } else {
        None
    };

    if let (Some(lon), Some(lon_ref)) = (longitude, lon_ref) {
        if let Some(lon_formatted) = format_gps_data(&lon, lon_ref) {
            println!("Longitude: {}", lon_formatted);
        }
    } else {
        println!("GPSLongitude or GPSLongitudeRef tag is missing.");
    }

    Ok(exif)
}

fn format_gps_data(rational: &Vec<exif::Rational>, ref_value: &str) -> Option<String> {
    if rational.len() == 3 {
        let degrees = rational[0].num as f64 / rational[0].denom as f64;
        let minutes = rational[1].num as f64 / rational[1].denom as f64;
        let seconds = rational[2].num as f64 / rational[2].denom as f64;
        Some(format!(
            "{:.0}°{:.0}'{:.2}\"{}",
            degrees, minutes, seconds, ref_value
        ))
    } else {
        None
    }
}
