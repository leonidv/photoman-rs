use std::io::BufReader;
use std::{fs::File, path::Path};

use exif::{Exif, Field, In, Tag};

use crate::error::Error;

#[derive(Debug)]
pub struct ImageInfo {
    path: String,
    datetime: String,
    path_datetime: String,
    camera: String,
}

fn get_field_or_error(exif: &Exif, tag: Tag) -> Result<&Field, Error> {
    let o_field = exif.get_field(tag, In::PRIMARY);
    match o_field {
        Some(field) => Ok(field),
        None => Err(Error::NoFieldError()),
    }
}

impl ImageInfo {
    pub fn read_exif<P>(file_path: P) -> Result<ImageInfo, Error>
    where
        P: AsRef<Path>,
    {
        let file = File::open(file_path.as_ref())?;
        let buff_capacity = file.metadata()?.len() as usize + 1; // see fs.rs initial_buffer_size
        let mut reader = BufReader::with_capacity(buff_capacity, file);
        let exif = exif::Reader::new().read_from_container(&mut reader)?;
        //let f_datetime = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)?;
        // let f_camera = exif.get_field(Tag::Model, In::PRIMARY)?;
        let f_datetime = get_field_or_error(&exif, Tag::DateTimeOriginal)?;
        let f_camera = get_field_or_error(&exif, Tag::Model)?;
        Ok(ImageInfo {
            path: String::from(file_path.as_ref().to_string_lossy()),
            datetime: f_datetime.display_value().to_string(),
            path_datetime: String::from(""),
            camera: f_camera.display_value().to_string(),
        })
    }
}
