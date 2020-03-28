extern crate warehouse;

use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use crate::error::Error;
use crate::dto::LogDto;
use warehouse::log::Log;

#[derive(Serialize, Deserialize, Debug)]
pub enum ExternalAction {
    Ping,
    StoreLog(LogDto),
    StoreLogs(Vec<LogDto>),
}

pub enum InternalAction {
    AddClient(SocketAddr),
}

pub enum Action {
    Internal(InternalAction),
    External(ExternalAction),
}

impl ExternalAction {
    pub fn to_action(self) -> Action {
        Action::External(self)
    }

    #[allow(dead_code)] // used for debugging
    pub fn serialize(self) -> Result<String, Error> {
        serde_json::to_string(&self)
            .or(Error::DeserializationFailed.as_result::<String>())
    }
}

impl Action {
    pub fn deserialize(source : &str) -> Result<Action, Error> {
        println!("{}", source);
        let external_action : ExternalAction = serde_json::from_str(source)
            .or(Error::DeserializationFailed.as_result::<ExternalAction>())?;
        Ok(external_action.to_action())
    }
}

pub fn run(action: Action) -> Result<(), Error> {
    match action {
        Action::External(ExternalAction::Ping) => {
            println!("{}", "received ping message");
            Ok(())
        },
        Action::Internal(InternalAction::AddClient(_)) => {
            // DEBUG
//            let action = ExternalAction::StoreLog(LogDto {
//                log_type: String::from("error"),
//                message: String::from("Some error"),
//                stack_trace: String::from("stack trace"),
//            });
            let action = ExternalAction::Ping;
            let json = action.serialize()?;
            println!("{}", json);

            println!("{}", "client connected");
            Ok(())
        },
        Action::External(ExternalAction::StoreLog(log)) => {
            let conn = Log::connection()
                .or(Err(Error::FailedDbConnection))?;

            log.to_log().persist(&conn)
                .or(Err(Error::FailedToWriteToDb))?;

            conn.close()
                .or(Err(Error::FailedToCloseDbConnection))
        },
        Action::External(ExternalAction::StoreLogs(logs)) => {
            let conn = Log::connection()
                .or(Err(Error::FailedDbConnection))?;

            logs
                .into_iter()
                .fold(Ok(()), |acc, log| acc.and_then(|_| log.to_log().persist(&conn)).map(|_| ()))
                .or(Err(Error::FailedToWriteToDb))?;

            conn.close()
                .or(Err(Error::FailedToCloseDbConnection))
        }
    }
}
