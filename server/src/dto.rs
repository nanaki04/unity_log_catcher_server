extern crate warehouse;

use serde::{Serialize, Deserialize};
use warehouse::log::Log;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogDto {
    log_type: String,
    message: String,
    stack_trace: String,
}

impl LogDto {
    pub fn to_log(self) -> Log {
        Log {
            id: 0,
            log_type: self.log_type,
            message: self.message,
            stack_trace: self.stack_trace,
        }
    }
}
