use std::{fs::File, io::BufReader};

use crate::error::Error;
use crate::exifreader::ExifData;

use super::ExifReader;
use chrono::NaiveDate;
use exif::{Exif, In, Tag};

pub struct RustReader;

impl ExifReader for RustReader {
    fn read<P>(&self, file_path: P) -> Result<ExifData, Error>
    where
        P: AsRef<std::path::Path> {
            let file = File::open(file_path.as_ref())?;
            let buff_capacity = file.metadata()?.len() as usize + 1; // see fs.rs initial_buffer_size
            let mut reader = BufReader::with_capacity(buff_capacity, file);
            let exif = exif::Reader::new().read_from_container(&mut reader)?;

            //let f_datetime = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)?;
            // let f_camera = exif.get_field(Tag::Model, In::PRIMARY)?;

            let f_datetime = get_field_or_error(&exif, Tag::DateTimeOriginal)?;
            let f_camera = get_field_or_error(&exif, Tag::Model)?;
            
            let f_date = f_datetime.split_once(' ').unwrap().0;
            

            Ok(ExifData {
                date: NaiveDate::parse_from_str(f_date, "%Y-%m-%d").unwrap(),
                camera: f_camera,
            })
    }
}


fn get_field_or_error(exif: &Exif, tag: Tag) -> Result<String, Error> {
    let o_field = exif.get_field(tag, In::PRIMARY);
    match o_field {
        Some(field) => Ok(field.display_value().to_string()),
        None => Err(Error::NoFieldError()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_from_jpeg() {
        let reader = RustReader{};
        let maybe_exif = reader.read("test_data/images/01.jpg");
        assert!(maybe_exif.is_ok());
        let exif = maybe_exif.unwrap();
        assert_eq!(exif.date, NaiveDate::from_ymd_opt(2020, 06, 21).unwrap()); 
    }
}