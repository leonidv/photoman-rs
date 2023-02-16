use std::{
    fs, io,
    path::{Path, PathBuf},
    collections::HashMap,
};

use chrono::NaiveDate;
use regex::Regex;


lazy_static! {
    static ref DATE_PREFIX: Regex = Regex::new(r"^(\d{4}-\d{2}-\d{2}).*").unwrap();
}

fn try_extract_date(path_str: &str) -> Option<NaiveDate> {
    let x: Option<regex::Captures> = DATE_PREFIX.captures_iter(path_str).next();
    return x.map(
        |c| NaiveDate::parse_from_str(&c[1], "%Y-%m-%d").unwrap(), // impossible error
    );
}

#[derive(Debug,Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum TargetType {
    IMAGE(NaiveDate),
    RAW(NaiveDate),
}

#[derive(Debug)]
pub(crate) struct Folders {
    pub source: Vec<PathBuf>,
    pub target: HashMap<TargetType,PathBuf>,
}

pub(crate) fn find_folders<P>(entry_point: &P, raw_folder: &str) -> io::Result<Folders>
where
    P: AsRef<Path>,
{
    let mut target_folders = HashMap::<TargetType,PathBuf>::new();
    let mut source_folders = Vec::new();


    for entry in fs::read_dir(entry_point)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(path_str) = entry.file_name().to_str() {
            if path.is_dir() {
                tracing::debug!(folder=path_str);
                match try_extract_date(path_str) {
                    Some(date) => {
                         target_folders.insert(TargetType::IMAGE(date), path.to_path_buf());
                         let raw_folder = path.join(raw_folder);
                         if raw_folder.is_dir() {
                            target_folders.insert(TargetType::RAW(date), path.to_path_buf());
                         }
                    },
                    None => source_folders.push(path),
                }
            }
        }
    }

    Ok(Folders {
        source: source_folders,
        target: target_folders,
    })
}