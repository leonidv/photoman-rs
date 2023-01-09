use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ExifError(exif::Error),
    NoFieldError(),
    WalkDirError()
}

impl From<exif::Error> for Error {
    fn from(error : exif::Error) -> Self {
        Error::ExifError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error : io::Error) -> Self {
        Error::Io(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(ref io_err) => io_err.fmt(f),
            Error::ExifError(exif_error) => exif_error.fmt(f),
            Error::NoFieldError() => f.write_str("field not found"),
            Error::WalkDirError() => f.write_str("cant walk dir")
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

// unstable API in rustc 1.49.0
// impl From<NoneError> for Error {
//     fn from(none: NoneError) -> Self {
//         Error::NoFieldError()
//     }
// }