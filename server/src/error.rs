use std::fmt;

#[derive(Debug)]
pub enum Error {
    ActionDispatchFailed,
    ActionReceiveFailed,
    ReceiveActionResponseFailed,
    FailedToOpenTcpListener,
    FailedToAcceptOnTcpListener,
    FailedToReadTcpStream,
    FailedToCreateTcpStreamWriter,
    FailedToSendResponse,
    CorruptTcpStreamData,
    EmptyTcpStreamData,
    DeserializationFailed,
    #[allow(dead_code)] // used for debugging
    SerializationFailed,
    FailedDbConnection,
    FailedToWriteToDb,
    FailedToReadFromDb,
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
