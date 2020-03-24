use std::io::{Error, ErrorKind};
use std::sync::mpsc::Receiver;

mod tcp;
mod actions;

fn main() {
    tcp::start_server()
        .map(handle_actions)
        .unwrap_or_else(|e| println!("{}", e));
}

fn handle_actions(receiver : Receiver<actions::Action>) {
    loop {
        receiver.recv()
            .or(Result::Err(Error::new(ErrorKind::Other, "Failed to receive action")))
            .and_then(actions::run)
            .unwrap_or_else(|e| println!("{}", e));
    }
}
