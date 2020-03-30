extern crate warehouse;

use serde::{Serialize, Deserialize};
use warehouse::log::{Log, LogType};
use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogDto {
    pub log_type: String,
    pub message: String,
    pub stack_trace: String,
}

impl LogDto {
    pub fn to_log(self) -> Result<Log, Error> {
        let log_type = LogType::from_string(self.log_type)
            .or(Error::InvalidLogType.as_result::<LogType>())?;

        Ok(Log {
            id: 0,
            log_type: log_type,
            message: self.message,
            stack_trace: self.stack_trace,
        })
    }
}
