use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::net::{TcpListener, TcpStream, SocketAddr, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::process::Command;
use regex::Regex;
use crate::actions::{Action, InternalAction, ExternalAction};
use crate::error::Error;

static ID_GEN: AtomicUsize = AtomicUsize::new(0);

pub fn start_server() -> Result<Receiver<Action>, Error> {
    let (tx, rx) = channel();
    let listener = TcpListener::bind("0.0.0.0:7033")
        .or(Error::FailedToOpenTcpListener.as_result::<TcpListener>())?;

    thread::spawn(move || {
        loop {
            accept(&listener, tx.clone())
                .unwrap_or_else(|e| println!("{}", e));
        }
    });

    let stdout = Command::new("hostname")
        .args(&["-I"])
        .output()
        .map(|output| output.stdout)
        .map(|stdout| String::from_utf8(stdout)
             .unwrap_or_else(|_| "".to_string())
        )
        .unwrap_or_else(|_| "".to_string());

    let ip_search = Regex::new(r"\s192.[^\s]+[\s|$]").unwrap();
    let ip = ip_search.captures(&stdout)
        .and_then(|caps| caps.get(0))
        .map(|cap| cap.as_str().trim())
        .unwrap_or_else(|| "0.0.0.0");

    println!("Listening on {}:7033", ip);

    Ok(rx)
}

fn accept(listener : &TcpListener, tx: Sender<Action>) -> Result<(), Error> {
    let (socket, _address) = listener.accept()
        .or(Error::FailedToAcceptOnTcpListener.as_result::<(TcpStream, SocketAddr)>())?;

    receive(socket, tx.clone())
}

fn receive(mut socket : TcpStream, tx : Sender<Action>) -> Result<(), Error> {
    let (tcp_out_tx, tcp_out_tr) = channel();

    let client_id = ID_GEN.fetch_add(1, Ordering::SeqCst);
    let action = Action::Internal(InternalAction::AddClient((client_id, tcp_out_tx.clone())));
    tx.send(action)
        .or(Error::ActionDispatchFailed.as_result::<()>())?;

    let mut write_stream = socket.try_clone()
        .or(Error::FailedToCreateTcpStreamWriter.as_result::<TcpStream>())?;

    thread::spawn(move || {
        let mut data = String::new();

        loop {
            let mut buffer = [0; 128];

            let result = socket.read(&mut buffer[..])
                .or(Error::FailedToReadTcpStream.as_result::<usize>())
                .and_then(|size| {
                    if size > 0 {
                        let incoming_data = from_utf8(&buffer[..size])
                            .or(Error::CorruptTcpStreamData.as_result::<&str>())?;

                        data.push_str(incoming_data);
                        take_messages(data)
                    } else {
                        let _ = tx.send(Action::External((ExternalAction::RemoveClient, client_id, tcp_out_tx.clone())));

                        Error::ClientDisconnected.as_result::<(String, Vec<String>)>()
                    }
                })
                .and_then(|(rest, messages)| {
                    let actions = parse_messages(messages, client_id, &tcp_out_tx)?;
                    actions
                        .into_iter()
                        .fold(Ok(()), |acc, action| acc.and_then(|_| tx.send(action)))
                        .map(|_| rest)
                        .or(Error::ActionDispatchFailed.as_result::<String>())
                });

            if result.is_err() {
                break;
            }

            data = result.unwrap();
        }
    });

    thread::spawn(move || {
        loop {
            let result = tcp_out_tr.recv()
                .or(Error::ReceiveActionResponseFailed.as_result())
                .and_then(|output| {
                    if output == "Close" {
                        Error::ClientDisconnected.as_result()
                    } else {
                        Ok(output)
                    }
                })
                .and_then(|output| write_stream.write(format!("{}\n", output.as_str()).as_bytes())
                    .or(Error::FailedToSendResponse.as_result())
                );

            if result.is_err() {
                let _ = write_stream.shutdown(Shutdown::Both);
                break;
            }

            println!("{}", "\nSending data:");
            println!("{}", result.unwrap());
            println!("{}", "\n");
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
        .ok_or(Error::EmptyTcpStreamData)
}

fn parse_messages(messages : Vec<String>, client_id : usize, sender : &Sender<String>) -> Result<Vec<Action>, Error> {
    println!("{}", "\nReceiving data:");
    messages
        .iter()
        .fold(Ok(Vec::<Action>::new()), |acc, message| {
            println!("{}", message);
            acc.and_then(|mut actions| {
                let action = Action::deserialize(message.as_str(), client_id, sender.clone())?;
                actions.push(action);
                Ok(actions)
            })
        })
}
