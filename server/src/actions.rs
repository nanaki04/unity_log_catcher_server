extern crate warehouse;

use serde::{Serialize, Deserialize};
use std::sync::mpsc::Sender;
use std::net::SocketAddr;
use crate::error::Error;
use crate::dto::LogDto;
use warehouse::log::Log;

#[derive(Serialize, Deserialize, Debug)]
pub enum ExternalAction {
    Ping(()),
    StoreLog(LogDto),
    StoreLogs(Vec<LogDto>),
    ReadLogs(i64),
    ClearLogs(()),
}

pub enum InternalAction {
    AddClient(SocketAddr),
}

pub enum Action {
    Internal(InternalAction),
    External((ExternalAction, Sender<String>)),
}

impl ExternalAction {
    pub fn to_action(self, sender: Sender<String>) -> Action {
        Action::External((self, sender))
    }

    #[allow(dead_code)] // used for debugging
    pub fn serialize(self) -> Result<String, Error> {
        let serialized = serde_json::to_string(&self)?;
        Ok(serialized)
    }
}

impl Action {
    pub fn deserialize(source: &str, sender: Sender<String>) -> Result<Action, Error> {
        let external_action : ExternalAction = serde_json::from_str(source)?;
        Ok(external_action.to_action(sender))
    }
}

pub fn run(action: Action) -> Result<(), Error> {
    match action {
        Action::External((ExternalAction::Ping(()), sender)) => {
            println!("{}", "received ping message");

            sender.send("pong!".to_string())
                .or(Err(Error::FailedToSendResponse))?;

            Ok(())
        },
        Action::Internal(InternalAction::AddClient(_)) => {
            // DEBUG
//            let action = ExternalAction::StoreLog(LogDto {
//                log_type: String::from("error"),
//                message: String::from("Some error"),
//                stack_trace: String::from("stack trace"),
//            });
            let action = ExternalAction::ClearLogs(());
            let json = action.serialize()?;
            println!("{}", json);

            println!("{}", "client connected");
            Ok(())
        },
        Action::External((ExternalAction::StoreLog(log), _)) => {
            let conn = Log::connection()?;

            log.to_log()?
                .persist(&conn)?;

            conn.close()
                .or(Err(Error::FailedToCloseDbConnection))?;

            Ok(())
        },
        Action::External((ExternalAction::StoreLogs(logs), _)) => {
            let conn = Log::connection()?;

            logs
                .into_iter()
                .map(|log| {
                    let log = log.to_log()?;
                    log.persist(&conn)?;
                    Ok(())
                })
                .collect::<Result<Vec<()>, Error>>()?;

            conn.close()
                .or(Err(Error::FailedToCloseDbConnection))?;

            Ok(())
        },
        Action::External((ExternalAction::ReadLogs(max), sender)) => {
            let conn = Log::connection()?;

            let logs : Vec<LogDto> = Log::fetch_with_limit(&conn, max)?
                .into_iter()
                .map(LogDto::from_log)
                .collect();

            let response = serde_json::to_string(&logs)?;

            sender.send(response)
                .or(Err(Error::FailedToSendResponse))?;

            Ok(())
        },
        Action::External((ExternalAction::ClearLogs(()), sender)) => {
            let conn = Log::connection()?;

            Log::truncate(&conn)?;

            sender.send("ok".to_string())
                .or(Err(Error::FailedToSendResponse))?;

            Ok(())
        },
    }
}
