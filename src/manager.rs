use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use chrono::NaiveDate;

use crate::{
    error::Error,
    exifreader::{ExifData, ExifReader},
    filesearch::{find_folders, TargetType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    IMAGE,
    RAW,
    MOVIE,
}

impl FileType {
    fn try_from_path<P: AsRef<Path>>(path: P, raw_exts: &Vec<String>) -> Option<FileType> {
        path.as_ref().extension().map(|ext| {
            let ext = ext.to_ascii_lowercase();
            if ext == "jpg" {
                // the most often case
                FileType::IMAGE
            } else if raw_exts.contains(&ext.to_string_lossy().to_string()) {
                FileType::RAW
            } else {
                FileType::IMAGE
            }
        })
    }
}

#[derive(Debug, Clone)]
struct FileInfo {
    path: PathBuf,
    exif: ExifData,
    f_type: FileType,
}

#[derive(Debug)]
pub(crate) struct Manager {
    work_dir: PathBuf,
    separate_raw: bool,
    raw_exts: Vec<String>,
    raw_folder: String,
    dry_run: bool,
}

#[derive(Debug)]
struct MoveFile {
    from: PathBuf,
    to: PathBuf,
}

#[derive(Debug)]
struct MkDir {
    target: PathBuf,
}

trait IOCommand {
    fn exec(&self, dry_run: bool) -> Result<(), Error>;
}

impl IOCommand for MoveFile {
    fn exec(&self, dry_run: bool) -> Result<(), Error> {
        if dry_run {
            let from = &self.from;
            let to = &self.to;
            println!("{} ➙ {}", from.to_string_lossy(), to.to_string_lossy());
            Ok(())
        } else {
            todo!()
        }
    }
}

impl IOCommand for MkDir {
    fn exec(&self, dry_run: bool) -> Result<(), Error> {
        if dry_run {
            println!(
                "create new directory: {}",
                self.target.as_path().to_string_lossy()
            );
            Ok(())
        } else {
            todo!()
        }
    }
}

struct FileProcessCommands {
    move_file: Option<MoveFile>,
    possible_mk_dir: Option<MkDir>,
}

impl FileProcessCommands {
    const EMPTY_FILE_COMMANDS: FileProcessCommands = FileProcessCommands {
        move_file: None,
        possible_mk_dir: None,
    };

    fn new_empty() -> FileProcessCommands {
        FileProcessCommands::EMPTY_FILE_COMMANDS
    }

    fn new(move_file: MoveFile, possible_mk_dir: Option<MkDir>) -> FileProcessCommands {
        FileProcessCommands {
            move_file: Some(move_file),
            possible_mk_dir,
        }
    }

    fn is_empty(&self) -> bool {
        self.move_file.is_none()
    }
}

// https://fileinfo.com/filetypes/camera_raw
const RAW_EXTENSIONS : &'static str = include_str!("../resources/raw_extensions");

impl Manager {
    pub(crate) fn new() -> Manager {
        let raw_exts = RAW_EXTENSIONS.lines().map(|l| l.to_ascii_lowercase().to_string()).collect();

        Manager {
            work_dir: PathBuf::from("."),
            separate_raw: true,
            raw_folder: "raw".to_string(),
            raw_exts,
            dry_run: false,
        }
    }

    pub(crate) fn work_dir<P: AsRef<Path>>(self, value: P) -> Manager {
        Manager {
            work_dir: value.as_ref().to_path_buf(),
            ..self
        }
    }

    pub(crate) fn dry_run(self) -> Manager {
        Manager {
            dry_run: true,
            ..self
        }
    }

    pub(crate) fn dont_separate_raw(self) -> Manager {
        Manager {
            separate_raw: false,
            ..self
        }
    }

    pub(crate) fn arrange_files(&mut self, exif_reader: &impl ExifReader) {
        println!("{:?}", self);

        let folders = find_folders(&self.work_dir, &self.raw_folder).unwrap();
        let sources = folders.source;

        let mut targets_per_date = folders.target;

        let mut mkdir_commands = Vec::<MkDir>::new();
        let mut move_commands = Vec::<MoveFile>::new();

        for source in sources {
            let process_result =
                self.process_source_folder(&source, &mut targets_per_date, exif_reader);
            match process_result {
                Ok(source_commands) => {
                    for sc in source_commands {
                        mkdir_commands.extend(sc.possible_mk_dir); // implicity unlift option
                        move_commands.extend(sc.move_file);
                    }
                }
                Err(e) => eprintln!(
                    "can't process [{}], error: {}]",
                    source.to_string_lossy(),
                    e
                ),
            };
        }

        println!("will create {} dirs", mkdir_commands.len());
        println!("will move {} images", move_commands.len());

        for mkdir in mkdir_commands {
            mkdir.exec(self.dry_run).unwrap()
        }

        for move_file in move_commands {
            move_file.exec(self.dry_run).unwrap()
        }
    }

    fn process_source_folder(
        &mut self,
        source: &PathBuf,
        targets_per_date: &mut HashMap<TargetType,PathBuf>,
        exif_reader: &impl ExifReader,
    ) -> Result<Vec<FileProcessCommands>, Error> {
        println!(
            "process source folder {}",
            source.as_path().to_string_lossy()
        );

        let mut commands = Vec::new();

        for _entry in source.read_dir()? {
            let entry = _entry?;
            let process_result = if entry.metadata()?.is_file() {
                let path = entry.path();
                match exif_reader.load(path.to_path_buf()) {
                    Ok(exif_data) => {
                        let f_type =
                            FileType::try_from_path(path.to_path_buf(), &self.raw_exts).unwrap_or(FileType::IMAGE);

                        let image_info = FileInfo {
                            exif: exif_data,
                            path: path.to_path_buf(),
                            f_type,
                        };

                        self.make_commands_to_process_image(targets_per_date, &image_info)
                    }
                    Err(e) => Err(e),
                }
            } else {
                Ok(FileProcessCommands::new_empty())
            };

            match process_result {
                Ok(file_commands) => {
                    if !file_commands.is_empty() {
                        commands.push(file_commands)
                    }
                }
                Err(e) => {
                    if let Error::Io(_) = e {
                        e.log(entry.path().to_path_buf());
                    }
                }
            }
        }

        return Ok(commands);
    }

    fn make_commands_to_process_image(
        &mut self,
        folder_per_date: &mut HashMap<TargetType, PathBuf>,
        file_info: &FileInfo,
    ) -> Result<FileProcessCommands, Error> {
        let image_path = &file_info.path;
        let exif = &file_info.exif;

        let image_name = match image_path.file_name() {
            Some(image_name) => image_name,
            None => return Ok(FileProcessCommands::new_empty()), // file_name == .. , do nothing
        };

        let put_in_raw_folder = file_info.f_type == FileType::RAW && self.separate_raw;

        let target_key = if put_in_raw_folder {
            TargetType::RAW(file_info.exif.date)
        } else {
            TargetType::IMAGE(file_info.exif.date)
        };

        let mut possible_mk_dir: Option<MkDir> = None;
        let date_dir = folder_per_date.entry(target_key).or_insert_with(|| {            
            let date = exif.date.format("%Y-%m-%d").to_string();
            let date_folder = self.work_dir.clone().join(date);
            let target = if put_in_raw_folder {
                date_folder.join(&self.raw_folder)
            } else {
                date_folder
            };

            possible_mk_dir = Some(MkDir {
                target: target.to_path_buf(),
            });

            target
        });

        // === still reoder in folders with dates not supported
        // === should be implemented by self.move_in_targets or something like
        
        let move_file = MoveFile {
            from: image_path.to_path_buf(),
            to: date_dir.clone().join(image_name),
        };
        Ok(FileProcessCommands::new(move_file, possible_mk_dir))
    }
}
