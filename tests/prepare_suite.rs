use lazy_static::lazy_static;

use std::{
    fs, io,
    path::{Path, PathBuf},
};


lazy_static! {
    static ref EXECUTION_TIMESTAMP: String = chrono::Local::now()
        .format("photoman_test_%Y-%m-%dT%H_%M_%S%.f")
        .to_string();
}

/// Prepare suite for testing.
///
/// Return test's working folder. Test can free modify content of this folder.
pub fn prepare_suite(test_name: &str) -> Result<PathBuf, io::Error> {
    let test_dir = std::env::temp_dir()
        .join(EXECUTION_TIMESTAMP.as_str())
        .join(test_name);

    copy_recursively("test_data/suite", &test_dir).map(|_| test_dir.to_path_buf())
}

/// Copy files from source to destination recursively.
fn copy_recursively<PS, PD>(source: PS, destination: PD) -> io::Result<()>
where
    PS: AsRef<Path>,
    PD: AsRef<Path>,
{
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
