use std::{
    fs, io,
    path::{self, Path, PathBuf},
    sync::Arc,
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

enum FolderType {
    WithDate(),
    ToSort(),
}

#[derive(Debug)]
pub struct SourceFolder {
   pub path: PathBuf,
}

#[derive(Debug)]
pub struct TargetFolder {
   pub path: PathBuf,
   pub date: NaiveDate,
}

#[derive(Debug)]
pub struct Folders {
   pub source: Vec<SourceFolder>,
   pub target: Vec<TargetFolder>,
}

pub fn find_folders<P>(entry_point: P) -> io::Result<Folders>
where
    P: AsRef<Path>,
{
    let mut result = Folders {
        source: Vec::new(),
        target: Vec::new(),
    };

    for entry in fs::read_dir(entry_point)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(path_str) = entry.file_name().to_str() {
            if path.is_dir() {
                match try_extract_date(path_str) {
                    Some(date) => result.target.push(TargetFolder { path, date }),
                    None => result.source.push(SourceFolder { path }),
                }
            }
        }
    }

    Ok(result)
}
