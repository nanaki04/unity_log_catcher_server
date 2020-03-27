extern crate warehouse;

use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use crate::error::Error;
use crate::dto::LogDto;
use warehouse::log::Log;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Ping,
    AddClient,
    StoreLog,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExternalPayload {
    Empty,
    Log(LogDto),
}

pub enum InternalPayload {
    Client(SocketAddr),
}

pub enum Payload {
    Internal(InternalPayload),
    External(ExternalPayload),
}

impl Payload {
    pub fn unwrap_log(self) -> Result<Log, Error> {
        match self {
            Payload::External(ExternalPayload::Log(log_dto)) => Ok(log_dto.to_log()),
            _ => Error::FailedToUnwrapLogPayload.as_result::<Log>(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalAction {
    pub command: Command,
    pub payload: ExternalPayload,
}

impl ExternalAction {
    pub fn to_action(self) -> Action {
        Action {
            command: self.command,
            payload: Payload::External(self.payload),
        }
    }

    #[allow(dead_code)] // used for debugging
    pub fn serialize(self) -> Result<String, Error> {
        serde_json::to_string(&self)
            .or(Error::DeserializationFailed.as_result::<String>())
    }
}

pub struct Action {
    pub command: Command,
    pub payload: Payload,
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
    match action.command {
        Command::Ping => {
            println!("{}", "received ping message");
            Ok(())
        },
        Command::AddClient => {
            // DEBUG
//            let action = ExternalAction {
//                command: Command::StoreLog,
//                payload: ExternalPayload::Log(LogDto {
//                    log_type: String::from("error"),
//                    message: String::from("Some error"),
//                    stack_trace: String::from("stack trace"),
//                }),
//            };
//
//            let json = action.serialize()?;
//            println!("{}", json);

            println!("{}", "client connected");
            Ok(())
        },
        Command::StoreLog => {
            let log = action.payload.unwrap_log()?;
            let conn = Log::connection()
                .or(Err(Error::FailedDbConnection))?;

            log.persist(&conn)
                .or(Err(Error::FailedToWriteToDb))?;

            conn.close()
                .or(Err(Error::FailedToCloseDbConnection))
        }
    }
}
