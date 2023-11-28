use exif::{Exif, In, Reader, Tag, Value};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    filename: String,
    date_created: Option<String>,
    gps_coordinates: Option<String>,
}

// https://github.com/kamadak/exif-rs/blob/master/examples/reading.rs
fn read_exif_data<P: AsRef<Path>>(file_path: P) -> Result<ImageData, String> {
    let path: &Path = file_path.as_ref();
    let exif: Exif = open_file(path)?;

    let date_created: Option<String> = get_date_time(&exif);
    let gps_coordinates: Option<String> = get_latitude(&exif)
        .and_then(|lat| get_longitude(&exif).map(|lon| format!("{}, {}", lat, lon)));

    Ok(ImageData {
        filename: path.file_name().unwrap().to_string_lossy().into_owned(),
        date_created,
        gps_coordinates,
    })
}

fn open_file<P: AsRef<Path>>(file_path: P) -> Result<Exif, String> {
    let mut file: File =
        File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    let mut buf: Vec<u8> = Vec::new();

    file.read_to_end(&mut buf)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    Reader::new()
        .read_from_container(&mut std::io::Cursor::new(&buf))
        .map_err(|e| format!("Failed to read EXIF data: {}", e))
}

fn get_date_time(exif: &Exif) -> Option<String> {
    exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)
        .and_then(|field: &exif::Field| match field.value {
            Value::Ascii(ref vec) if !vec.is_empty() => {
                std::str::from_utf8(&vec[0]).ok().map(String::from)
            }
            _ => None,
        })
}

fn get_latitude(exif: &Exif) -> Option<String> {
    let latitude: Option<Vec<exif::Rational>> = exif
        .get_field(Tag::GPSLatitude, In::PRIMARY)
        .and_then(|field: &exif::Field| match &field.value {
            Value::Rational(lat_values) => Some(lat_values.clone()),
            _ => None,
        });

    let lat_ref: Option<&str> =
        exif.get_field(Tag::GPSLatitudeRef, In::PRIMARY)
            .and_then(|field| match field.value {
                Value::Ascii(ref vec) => std::str::from_utf8(&vec[0]).ok(),
                _ => None,
            });

    if let (Some(lat), Some(lat_ref)) = (latitude, lat_ref) {
        format_gps_data(&lat, lat_ref)
    } else {
        None
    }
}

fn get_longitude(exif: &Exif) -> Option<String> {
    let longitude: Option<Vec<exif::Rational>> = exif
        .get_field(Tag::GPSLongitude, In::PRIMARY)
        .and_then(|field: &exif::Field| match &field.value {
            Value::Rational(lon_values) => Some(lon_values.clone()),
            _ => None,
        });

    let lon_ref: Option<&str> = exif.get_field(Tag::GPSLongitudeRef, In::PRIMARY).and_then(
        |field: &exif::Field| match field.value {
            Value::Ascii(ref vec) => std::str::from_utf8(&vec[0]).ok(),
            _ => None,
        },
    );

    if let (Some(lon), Some(lon_ref)) = (longitude, lon_ref) {
        format_gps_data(&lon, lon_ref)
    } else {
        None
    }
}

fn format_gps_data(rational: &Vec<exif::Rational>, ref_value: &str) -> Option<String> {
    if rational.len() == 3 {
        let degrees = rational[0].num as f64 / rational[0].denom as f64;
        let minutes = rational[1].num as f64 / rational[1].denom as f64;
        let seconds = rational[2].num as f64 / rational[2].denom as f64;
        let decimal = degrees + (minutes / 60.0) + (seconds / 3600.0);

        let corrected_decimal = match ref_value {
            "N" | "E" => decimal,
            "S" | "W" => -decimal,
            _ => decimal,
        };

        Some(format!("{:.6}", corrected_decimal))
    } else {
        None
    }
}

pub fn process_images<P: AsRef<Path>>(folder_path: P) -> Result<(), String> {
    let paths: fs::ReadDir =
        fs::read_dir(folder_path).map_err(|e: std::io::Error| e.to_string())?;

    let mut images_data: Vec<ImageData> = Vec::new();

    for path in paths {
        let path: std::path::PathBuf = path.map_err(|e: std::io::Error| e.to_string())?.path();
        if image_check(&path) {
            match read_exif_data(&path) {
                Ok(data) => images_data.push(data),
                Err(e) => eprintln!("Error reading EXIF data: {}", e),
            }
        }
    }

    images_data = image_sort(images_data);

    let json: String =
        serde_json::to_string_pretty(&images_data).map_err(|e: serde_json::Error| e.to_string())?;

    fs::write("output.json", json).map_err(|e: std::io::Error| e.to_string())?;

    Ok(())
}

// https://stackoverflow.com/questions/72392835/check-if-a-file-is-of-a-given-type
fn image_check(path: &Path) -> bool {
    match path.extension().and_then(std::ffi::OsStr::to_str) {
        Some(ext) => {
            ext.eq_ignore_ascii_case("jpg")
                || ext.eq_ignore_ascii_case("jpeg")
                || ext.eq_ignore_ascii_case("tiff")
                || ext.eq_ignore_ascii_case("tif")
                || ext.eq_ignore_ascii_case("png")
        }
        None => false,
    }
}

// https://stackoverflow.com/questions/56105305/how-to-sort-a-vec-of-structs-by-a-string-field
fn image_sort(mut images: Vec<ImageData>) -> Vec<ImageData> {
    images.sort_by(
        |a: &ImageData, b: &ImageData| match (&a.date_created, &b.date_created) {
            (Some(date_a), Some(date_b)) => date_a.cmp(date_b),
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (None, None) => std::cmp::Ordering::Equal,
        },
    );
    images
}