use std::{fs, path::PathBuf};

use crate::error::Error;

#[derive(Debug)]
pub(crate) struct MoveFile {
    pub(crate) from: PathBuf,
    pub(crate) to: PathBuf,
}

#[derive(Debug)]
pub(crate) struct MkDir {
    pub(crate) target: PathBuf,
}

pub(crate) struct RmEmptyDir {
    pub(crate) target: PathBuf,
}

pub(crate) trait IOCommand {
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
            fs::rename(&self.from, &self.to).map_err(Error::from)
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
            fs::create_dir_all(&self.target).map_err(Error::from)
        }
    }
}

impl IOCommand for RmEmptyDir {
    fn exec(&self, dry_run: bool) -> Result<(), Error> {
        if !dry_run {
            fs::remove_dir(&self.target).map_err(Error::from)
        } else {
            Ok(())
        }
    }
}
