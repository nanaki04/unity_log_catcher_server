use std::fmt;

#[derive(Debug)]
pub enum Error {
    RusqLiteError(rusqlite::Error),
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
