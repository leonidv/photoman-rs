use std::{fs::File, io::BufReader, path::Path};

use crate::error::Error;
use crate::exifreader::ExifData;

use super::ExifReader;
use chrono::NaiveDate;
use exif::{Exif, In, Tag};
use tracing::trace;

pub struct RustReader;

impl ExifReader for RustReader {
    fn read<P>(&self, file_path: P) -> Result<ExifData, Error>
    where
        P: AsRef<std::path::Path>,
    {
        let file = File::open(file_path.as_ref())?;
        let buff_capacity = file.metadata()?.len() as usize + 1; // see fs.rs initial_buffer_size
        let mut reader = BufReader::with_capacity(buff_capacity, file);
        let exif = exif::Reader::new().read_from_container(&mut reader)?;


        let f_datetime = get_field_or_error(&exif, Tag::DateTimeOriginal)?;
        //let f_camera = get_field_or_error(&exif, Tag::Model)?;

        trace!("exif: {}", f_datetime);

        let f_date = f_datetime.split_once(' ').unwrap().0;

        Ok(ExifData {
            date: NaiveDate::parse_from_str(f_date, "%Y-%m-%d").unwrap()
        })
    }
}

// For manual debugging. See test dump
#[allow(unused)] 
fn dump_file<P: AsRef<Path>>(path: P) -> Result<(), exif::Error> {
    let file = File::open(&path)?;
    let exif = exif::Reader::new().read_from_container(&mut BufReader::new(&file))?;

    println!("{}", path.as_ref().display());
    for f in exif.fields() {
        let display_value = f.display_value().to_string();
        if display_value.len() > 100 {
            println!("  {}/{}: [...]", f.ifd_num.index(), f.tag);
        } else {
            println!("  {}/{}[{:#x}]: {}", f.ifd_num.index(), f.tag, f.tag.number(), display_value);
            println!("      {:?}", f.value);
        };
    }
    Ok(())
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
        let reader = RustReader {};
        let maybe_exif = reader.read("test_data/images/01.jpg");
        assert!(maybe_exif.is_ok());
        let exif = maybe_exif.unwrap();
        assert_eq!(exif.date, NaiveDate::from_ymd_opt(2020, 06, 21).unwrap());
    }

    #[test]
    fn read_from_generated_exif() {
        let reader = RustReader {};
        let maybe_exif = reader.read("test_data/images/000000581894.jpg");
        assert!(maybe_exif.is_ok());
        let exif = maybe_exif.unwrap();
        assert_eq!(exif.date, NaiveDate::from_ymd_opt(2021, 04, 22).unwrap());

    }

    //#[test]
    #[allow(unused)]
    fn dump() {
        dump_file("test_data/images/01.jpg").unwrap();
        dump_file("test_data/images/000000581894.jpg").unwrap();
    }
}
