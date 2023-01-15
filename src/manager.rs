use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use chrono::NaiveDate;

use crate::{
    error::Error,
    exifreader::{ExifReader, ImageInfo},
    filesearch::{find_folders, Folders, SourceFolder, TargetFolder},
};

pub(crate) struct Manager {
    work_dir: PathBuf,
    dry_run: bool,
    // targets: Vec<TargetFolder>,
    // sources: &'a [SourceFolder],
    // target_by_date: HashMap<NaiveDate, PathBuf>,
}

impl Manager {
    pub(crate) fn new<'a, P: AsRef<Path>>(work_dir: P, dry_run: bool) -> Result<Manager, Error> {
        let folders = find_folders(&work_dir)?;
        let targets = folders.target;
        let sources = folders.source;

        let mut target_by_date = HashMap::new();

        for target in &targets {
            target_by_date.insert(target.date, target.path.clone());
        }

        return Ok(Manager {
            work_dir: work_dir.as_ref().to_path_buf(),
            dry_run,
            // targets,
            // sources: sources,
            // target_by_date,
        });
    }

    pub(crate) fn arrange_files(&mut self, exif_reader: &impl ExifReader) {
        let folders = find_folders(&self.work_dir).unwrap();
        let targets = folders.target;
        let sources = folders.source;

        let mut target_by_date = HashMap::new();

        for target in &targets {
            target_by_date.insert(target.date, target.path.clone());
        }

        for source in sources {
            self.process_source_folder(&source.path, &mut target_by_date, exif_reader)
                .unwrap();
        }
    }

    fn process_source_folder(
        &mut self,
        source: &PathBuf,
        target_by_date: &mut HashMap<NaiveDate, PathBuf>,
        exif_reader: &impl ExifReader,
    ) -> Result<(), Error> {
        println!(
            "process source folder {}",
            source.as_path().to_string_lossy()
        );

        for _entry in source.read_dir()? {
            let entry = _entry?;
            let process_result = if entry.metadata()?.is_file() {
                match exif_reader.load(entry.path()) {
                    Ok(image_info) => self.process_image(target_by_date, &image_info),
                    Err(e) => Err(e),
                }
            } else {
                Ok(())
            };

            if let Err(e) = process_result {
                match e {
                    Error::Io(_) => eprintln!(
                        "Can't process [{}], error: {}",
                        entry.path().to_string_lossy(),
                        e
                    ),
                    _ => {}
                }
            }
        }

        return Ok(());
    }

    fn process_image(
        &mut self,
        target_by_date: &mut HashMap<NaiveDate, PathBuf>,
        image_info: &ImageInfo,
    ) -> Result<(), Error> {
        let target_dir = target_by_date.entry(image_info.date).or_insert_with(|| {
            let date = image_info.date.format("%Y-%m-%d").to_string();
            self.work_dir.clone().join(date)
        });

        let image_name = image_info.path.file_name().unwrap();
        let target_file = target_dir.clone().join(image_name);

        if self.dry_run {
            println!(
                "{} ➙ {}",
                image_info.path.to_string_lossy(),
                target_file.to_string_lossy()
            )
        } else {
            todo!();
        }

        Ok(())
        //println!("{} ➙ {}",source.to_string_lossy(), target.to_string_lossy())
    }
}
