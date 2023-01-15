use std::path::{Path, PathBuf};

use chrono::NaiveDate;

use crate::error::Error;

use self::rustreader::RustReader;
mod rustreader;

pub enum ImageType {
    IMAGE,
    RAW,
    MOVIE
}

#[derive(Debug)]
pub struct ImageInfo {
    path: PathBuf,
    date: NaiveDate,
    camera: String,
}

pub trait ExifReader {
    fn load<P>(&self, file_path: P) -> Result<ImageInfo, crate::error::Error>
    where
        P: AsRef<Path>;
}


pub fn create_exif_reader() -> impl ExifReader {
    RustReader{}
}
