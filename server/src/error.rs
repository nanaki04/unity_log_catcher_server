use std::fmt;

#[derive(Debug)]
pub enum Error {
    ActionDispatchFailed,
    ActionReceiveFailed,
    FailedToOpenTcpListener,
    FailedToAcceptOnTcpListener,
    FailedToReadTcpStream,
    CorruptTcpStreamData,
    EmptyTcpStreamData,
    DeserializationFailed,
    FailedToUnwrapLogPayload,
    FailedDbConnection,
    FailedToWriteToDb,
    FailedToCloseDbConnection,
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
