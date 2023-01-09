use walkdir::{WalkDir, DirEntry};
use crate::error::Error;
use std::fs::File;
use std::ffi::OsStr;
use std::collections::HashMap;

pub struct FileInfo {
    path: String,
    is_raw: bool,
}

enum FileType {
    NOT_INTERESTED,
    DIR,
    IMAGE,
    RAW,
    VIDEO,
}

const IMAGE_EXTENSIONS: [&str; 2] = ["JPG", "JPEG"];

const RAW_EXTENSIONS: [&str; 2] = ["PEF", "ARW"];

const map: HashMap<&str, FileType> =
    [
        ("JPG", FileType::IMAGE),
        ("JPEG", FileType::IMAGE),
        ("PEF", FileType::RAW),
        ("ARW", FileType::RAW),
        ("MOV", FileType::VIDEO)
    ].iter().cloned().collect();


pub fn find_images(path: String) -> Vec<FileInfo> {}

fn get_file_type(dir_entry: &DirEntry) -> Result<FileType, Error> {
    if dir_entry.metadata()?.is_dir() {
        return Ok(FileType::DIR);
    } else {
        let x =
            dir_entry.path().extension()
                .map(|ext| extension_to_file_type(ext))
                .ok_or(Err(Error::WalkDirError()))?;
        return x;
    }
}

fn extension_to_file_type(ext: &OsStr) -> Result<FileType, Error> {
    return match ext.to_str() {
        Some()
    };
}