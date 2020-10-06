use std::fmt;

#[derive(Debug)]
pub enum Error {
    DbError(warehouse::error::Error),
    SerializationError(serde_json::error::Error),
    ActionDispatchFailed,
    ActionReceiveFailed,
    ReceiveActionResponseFailed,
    FailedToOpenTcpListener,
    FailedToAcceptOnTcpListener,
    FailedToReadTcpStream,
    FailedToCreateTcpStreamWriter,
    FailedToSendResponse,
    ClientDisconnected,
    CorruptTcpStreamData,
    EmptyTcpStreamData,
    #[allow(dead_code)] // used for debugging
    SerializationFailed,
    DeserializationFailed,
    FailedDbConnection,
    FailedToCloseDbConnection,
    InvalidLogType,
}

impl Error {
    pub fn as_result<T>(self) -> Result<T, Error> {
        Err(self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<warehouse::error::Error> for Error {
    fn from(error: warehouse::error::Error) -> Self {
        Error::DbError(error)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::SerializationError(error)
    }
}
