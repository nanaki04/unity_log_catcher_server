use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::net::{TcpListener, TcpStream};
use std::io::{Error, ErrorKind, Read};
use crate::actions::{Action, Command, Payload, InternalPayload};

pub fn start_server() -> Result<Receiver<Action>, Error> {
    let (tx, rx) = channel();
    let listener = TcpListener::bind("127.0.0.1:7033")?;

    thread::spawn(move || {
        loop {
            accept(&listener, tx.clone())
                .unwrap_or_else(|e| println!("{}", e));
        }
    });

    Ok(rx)
}

fn accept(listener : &TcpListener, tx: Sender<Action>) -> Result<(), Error> {
    let (socket, address) = listener.accept()?;

    let action = Action {
        command: Command::AddClient,
        payload: Payload::Internal(InternalPayload::Client(address)),
    };

    tx.send(action)
        .or(Result::Err(Error::new(ErrorKind::Other, "Failed to send from tcp to main thread")))?;

    receive(socket, tx.clone())
}

fn receive(mut socket : TcpStream, tx : Sender<Action>) -> Result<(), Error> {
    thread::spawn(move || {
        let mut data = String::new();

        loop {
            let mut buffer = [0; 128];

            data = socket.read(&mut buffer[..])
                .and_then(|size| {
                    if size > 0 {
                        take_messages(data)
                    } else {
                        Ok((data, Vec::<String>::new()))
                    }
                })
                .and_then(|(rest, messages)| {
                    let actions = parse_messages(messages)?;
                    actions
                        .into_iter()
                        .fold(Ok(()), |acc, action| acc.and_then(|_| tx.send(action)))
                        .map(|_| rest)
                        .or(Result::Err(Error::new(ErrorKind::Other, "Failed to send action")))
                })
                .unwrap_or_else(|e| {
                    println!("{}", e);
                    String::new()
                });
        }
    });

    Ok(())
}

fn take_messages(data : String) -> Result<(String, Vec<String>), Error> {
    let chunks : Vec<&str> = data.split("\n")
        .collect();

    chunks.split_last()
        .map(|(rest, messages)| (
            String::from(*rest),
            messages.iter()
                .map(|message| String::from(*message))
                .collect())
        )
        .ok_or(Error::new(ErrorKind::Other, "Invalid tcp data"))
}

fn parse_messages(messages : Vec<String>) -> Result<Vec<Action>, Error> {
    messages
        .iter()
        .fold(Ok(Vec::<Action>::new()), |acc, message| {
            acc.and_then(|mut actions| {
                let action = Action::deserialize(message.as_str())?;
                actions.push(action);
                Ok(actions)
            })
        })
}
