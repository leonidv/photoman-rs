#[macro_use]
extern crate lazy_static;

mod error;
mod exifreader;
mod filesearch;
mod iocommands;
mod progress;

use std::{
    fs::DirEntry,
    io,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::{
    error::Error,
    exifreader::{create_exif_reader, ExifData, ExifReader},
    filesearch::{find_folders, TargetType},
    iocommands::*,
};

use dashmap::DashMap;
use rayon::prelude::*;
use tracing::{debug, debug_span, info, span, trace, warn, Level};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    IMAGE,
    RAW,
    MOVIE,
}

impl FileType {
    /**
     * Detect type of file - image, raw, movie.
     *
     * Return None if path does not contain extension.
     */
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
pub struct Manager {
    work_dir: PathBuf,

    #[allow(dead_code)]
    use_exiftool: bool,
    separate_raw: bool,
    raw_folder: String,
    dry_run: bool,
    raw_exts: Vec<String>,
}

struct FileProcessing {
    move_file: Option<MoveFile>,
    mk_dir: Option<MkDir>,
}

impl FileProcessing {
    const EMPTY_FILE_COMMANDS: FileProcessing = FileProcessing {
        move_file: None,
        mk_dir: None,
    };

    fn new_empty() -> FileProcessing {
        FileProcessing::EMPTY_FILE_COMMANDS
    }

    fn new(move_file: MoveFile, possible_mk_dir: Option<MkDir>) -> FileProcessing {
        FileProcessing {
            move_file: Some(move_file),
            mk_dir: possible_mk_dir,
        }
    }

    fn is_empty(&self) -> bool {
        self.move_file.is_none()
    }
}

// https://fileinfo.com/filetypes/camera_raw
const RAW_EXTENSIONS: &'static str = include_str!("../resources/raw_extensions");

unsafe impl Sync for Manager {}

impl Manager {
    pub fn new() -> Manager {
        let raw_exts = RAW_EXTENSIONS
            .lines()
            .map(|l| l.to_ascii_lowercase().to_string())
            .collect();

        Manager {
            work_dir: PathBuf::from("."),
            use_exiftool: false,
            separate_raw: true,
            raw_folder: "raw".to_string(),
            raw_exts,
            dry_run: false,
        }
    }

    pub fn work_dir<P: AsRef<Path>>(self, value: P) -> Manager {
        Manager {
            work_dir: value.as_ref().to_path_buf(),
            ..self
        }
    }

    pub fn dry_run(self) -> Manager {
        Manager {
            dry_run: true,
            ..self
        }
    }

    pub fn dont_separate_raw(self) -> Manager {
        Manager {
            separate_raw: false,
            ..self
        }
    }

    #[tracing::instrument(skip(self), level=Level::DEBUG)]
    pub fn arrange_files(&mut self) {
        tracing::debug!(?self);

        let exif_reader = create_exif_reader();

        // !!! PERFORMANCE: find_folders
        let span = span!(Level::DEBUG, "find_folders").entered();
        let folders = find_folders(&self.work_dir, &self.raw_folder).unwrap();
        span.exit();

        let sources = folders.source;

        let targets_per_date = Arc::new(folders.target);

        let mut mkdir_commands = Vec::<MkDir>::new();
        let mut move_commands = Vec::<MoveFile>::new();

        // !!! PERFORMANCE: make commands
        let span = debug_span!("make_commands").entered();
        let progress_indicator =
            progress::ProgressIndicator::new(sources.len(), "read metadata from folders".to_string());

        for source in sources.iter() {
            let process_result =
                self.prepare_commands_for_folder(&source, &targets_per_date, &exif_reader);
            match process_result {
                Ok(source_commands) => {
                    for sc in source_commands {
                        mkdir_commands.extend(sc.mk_dir); // implicity unlift option
                        move_commands.extend(sc.move_file);
                    }
                }
                Err(e) => warn!(
                    "can't process [{}], error: {}]",
                    source.to_string_lossy(),
                    e
                ),
            };

            progress_indicator.step();
        }

        span.exit();

        debug!("will create {} dirs", mkdir_commands.len());
        debug!("will move {} images", move_commands.len());

        // !!! PERFORMANCE: make directories
        let span = debug_span!("mkdir").entered();
        for mkdir in mkdir_commands {
            mkdir.exec(self.dry_run).unwrap()
        }
        span.exit();

        // !!! PERFORMANCE: move files
        let total_images_move = move_commands.len();
        let progress_indicator =
            progress::ProgressIndicator::new(total_images_move, "moving images ".to_string());
        let span = debug_span!("move images").entered();

        // lv try par
        move_commands.par_iter().for_each(|move_file| {
            move_file.exec(self.dry_run).unwrap();
            progress_indicator.step();
        });

        span.exit();

        for source in &sources {
            let cmd = RmEmptyDir {
                target: source.to_path_buf(),
            };
            
               

            match cmd.exec(self.dry_run) {
                Ok(_) => info!("Removed empty folder {}", source.to_string_lossy()),
                Err(e) => warn!("Can't remove folder {}, error {}", source.to_string_lossy(),  e),
            }
        }
    }

    // show warning and return None
    fn warn_io_error<T, S, P>(operation: S, e: Error, path: P) -> Option<T>
    where
        S: AsRef<str>,
        P: AsRef<Path>,
    {
        match e {
            Error::Io(io_err) => {
                let os_error: String = io_err
                    .raw_os_error()
                    .map_or("".to_string(), |code| format!(", os error: {}", code)); // ugly code
                warn!(
                    "{}, path [{}], e = {}, os_error = {}",
                    operation.as_ref(),
                    path.as_ref().to_string_lossy(),
                    io_err,
                    os_error
                )
            }
            _ => warn!(
                "{}, path [{}], e = {}",
                operation.as_ref(),
                path.as_ref().to_string_lossy(),
                e
            ),
        }
        None
    }

    #[tracing::instrument(skip_all, level=Level::TRACE )]
    fn prepare_commands_for_folder(
        &mut self,
        source_folder: &PathBuf,
        targets_per_date: &Arc<DashMap<TargetType, PathBuf>>,
        exif_reader: &impl ExifReader,
    ) -> Result<Vec<FileProcessing>, Error> {
        let dir_name = source_folder.as_path().to_string_lossy().to_string();
        trace!(
            "read EXIF from images in {}",
            source_folder.as_path().to_string_lossy()
        );

        let span = debug_span!("getting list of files", folder = dir_name).entered();
        let mut files_in_folder = Vec::<DirEntry>::new();
        for _entry in source_folder.read_dir()? {
            files_in_folder.push(_entry?);
        }
        span.exit();

        let progress_indicator = progress::ProgressIndicator::new(
            files_in_folder.len(),
            format!("read from images in folder {}: ", dir_name),
        );

        let x = files_in_folder
            .par_iter()
            .filter_map(|dir_entry| match dir_entry.metadata() {
                Ok(metadata) => {
                    if metadata.is_file() {
                        Some(dir_entry)
                    } else {
                        None
                    }
                }

                Err(e) => {
                    Self::warn_io_error("Can't read metadata", Error::Io(e), dir_entry.path())
                }
            })
            .filter_map(|dir_entry| {
                FileType::try_from_path(dir_entry.path(), &self.raw_exts)
                    .map(|file_type| (dir_entry, file_type))
            })
            .filter_map(
                |(dir_entry, file_type)| match exif_reader.read(dir_entry.path()) {
                    Ok(exif_data) => Some(FileInfo {
                        exif: exif_data,
                        path: dir_entry.path(),
                        f_type: file_type,
                    }),

                    Err(e) => Self::warn_io_error("Can't read EXIF", e, dir_entry.path()),
                },
            )
            .filter_map(|file_info| {
                let r = match self.make_commands_to_process_image(targets_per_date, &file_info) {
                    Ok(commands) => Some(commands),
                    Err(e) => Self::warn_io_error("Can't prepare commands", e, file_info.path),
                };
                progress_indicator.step();
                r
            });

        let v: Vec<FileProcessing> = x.collect();


        return Ok(v);
    }

    // Analyze image and make required commands. One image may require moving file and creating
    // new directory
    fn make_commands_to_process_image(
        &self,
        folder_per_date: &Arc<DashMap<TargetType, PathBuf>>,
        file_info: &FileInfo,
    ) -> Result<FileProcessing, Error> {
        let image_path = &file_info.path;
        let exif = &file_info.exif;

        let image_name = match image_path.file_name() {
            Some(image_name) => image_name,
            None => return Ok(FileProcessing::new_empty()), // file_name == .. , do nothing
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

        // should not overwrite an existing file
        let target_filename = date_dir.join(image_name);
        if !target_filename.exists() {
            let move_file = MoveFile {
                from: image_path.to_path_buf(),
                to: target_filename,
            };
            Ok(FileProcessing::new(move_file, possible_mk_dir))
        } else {
            Ok(FileProcessing::new_empty())
        }
    }
}
