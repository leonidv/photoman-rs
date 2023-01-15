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
pub(crate) struct ImageInfo {
   pub path: PathBuf,
   pub date: NaiveDate,
   pub camera: String,
}

pub(crate) trait ExifReader {
    fn load<P>(&self, file_path: P) -> Result<ImageInfo, crate::error::Error>
    where
        P: AsRef<Path>;
}


pub(crate) fn create_exif_reader() -> impl ExifReader {
    RustReader{}
}
