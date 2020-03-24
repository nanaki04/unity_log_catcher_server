use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Ping,
    AddClient,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExternalPayload {
    Empty,
}

pub enum InternalPayload {
    Client(SocketAddr),
}

pub enum Payload {
    Internal(InternalPayload),
    External(ExternalPayload),
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
}

pub struct Action {
    pub command: Command,
    pub payload: Payload,
}

impl Action {
    pub fn deserialize(source : &str) -> Result<Action, Error> {
        let external_action : ExternalAction = serde_json::from_str(source)?;
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
            Ok(())
        }
    }
}
