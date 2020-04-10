use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::str::from_utf8;
use crate::actions::{Action, InternalAction};
use crate::error::Error;

pub fn start_server() -> Result<Receiver<Action>, Error> {
    let (tx, rx) = channel();
    let listener = TcpListener::bind("127.0.0.1:7033")
        .or(Error::FailedToOpenTcpListener.as_result::<TcpListener>())?;

    thread::spawn(move || {
        loop {
            accept(&listener, tx.clone())
                .unwrap_or_else(|e| println!("{}", e));
        }
    });

    Ok(rx)
}

fn accept(listener : &TcpListener, tx: Sender<Action>) -> Result<(), Error> {
    let (socket, address) = listener.accept()
        .or(Error::FailedToAcceptOnTcpListener.as_result::<(TcpStream, SocketAddr)>())?;

    let action = Action::Internal(InternalAction::AddClient(address));
    tx.send(action)
        .or(Error::ActionDispatchFailed.as_result::<()>())?;

    receive(socket, tx.clone())
}

fn receive(mut socket : TcpStream, tx : Sender<Action>) -> Result<(), Error> {
    let (tcp_out_tx, tcp_out_tr) = channel();
    let mut write_stream = socket.try_clone()
        .or(Error::FailedToCreateTcpStreamWriter.as_result::<TcpStream>())?;

    thread::spawn(move || {
        let mut data = String::new();

        loop {
            let mut buffer = [0; 128];

            data = socket.read(&mut buffer[..])
                .or(Error::FailedToReadTcpStream.as_result::<usize>())
                .and_then(|size| {
                    if size > 0 {
                        let incoming_data = from_utf8(&buffer[..size])
                            .or(Error::CorruptTcpStreamData.as_result::<&str>())?;

                        data.push_str(incoming_data);
                        take_messages(data)
                    } else {
                        Ok((data, Vec::<String>::new()))
                    }
                })
                .and_then(|(rest, messages)| {
                    let actions = parse_messages(messages, &tcp_out_tx)?;
                    actions
                        .into_iter()
                        .fold(Ok(()), |acc, action| acc.and_then(|_| tx.send(action)))
                        .map(|_| rest)
                        .or(Error::ActionDispatchFailed.as_result::<String>())
                })
                .unwrap_or_else(|e| {
                    println!("{}", e);
                    String::new()
                });
        }
    });

    thread::spawn(move || {
        loop {
            tcp_out_tr.recv()
                .or(Error::ReceiveActionResponseFailed.as_result())
                .and_then(|output| write_stream.write(format!("{}\n", output.as_str()).as_bytes())
                    .or(Error::FailedToSendResponse.as_result())
                    .map(|_| ())
                )
                .unwrap_or_else(|e| {
                    println!("{}", e);
                });
        }
    });

    Ok(())
}

fn take_messages(data : String) -> Result<(String, Vec<String>), Error> {
    println!("{}", "take messages");
    let chunks : Vec<&str> = data.split("\n")
        .collect();

    chunks.split_last()
        .map(|(rest, messages)| (
            String::from(*rest),
            messages.iter()
                .map(|message| String::from(*message))
                .collect())
        )
        .ok_or(Error::EmptyTcpStreamData)
}

fn parse_messages(messages : Vec<String>, sender : &Sender<String>) -> Result<Vec<Action>, Error> {
    println!("{}", "parse messages");
    messages
        .iter()
        .fold(Ok(Vec::<Action>::new()), |acc, message| {
            println!("{}", message);
            acc.and_then(|mut actions| {
                let action = Action::deserialize(message.as_str(), sender.clone())?;
                actions.push(action);
                Ok(actions)
            })
        })
}
