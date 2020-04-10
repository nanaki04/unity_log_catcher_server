use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    RusqLiteError(rusqlite::Error),
    IoError(io::Error),
    FailedToConvertLogType,
}

impl Error {
    pub fn failed_to_convert_log_type<T>() -> Result<T, Error> {
        Err(Error::FailedToConvertLogType)
    }

    pub fn rusqlite_error<T>(error: rusqlite::Error) -> Result<T, Error> {
        Err(Error::RusqLiteError(error))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        Error::RusqLiteError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}
