use std::path::{Path};

use chrono::NaiveDate;


use self::rustreader::RustReader;
mod rustreader;


#[derive(Debug,Clone)]
pub(crate) struct ExifData {
   pub date: NaiveDate,
   pub camera: String,
}

pub(crate) trait ExifReader {
    fn load<P>(&self, file_path: P) -> Result<ExifData, crate::error::Error>
    where
        P: AsRef<Path>;
}


pub(crate) fn create_exif_reader() -> impl ExifReader {
    RustReader{}
}
