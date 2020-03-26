use std::sync::mpsc::Receiver;

mod tcp;
mod dto;
mod actions;
mod error;

fn main() {
    tcp::start_server()
        .map(handle_actions)
        .unwrap_or_else(|e| println!("{}", e));
}

fn handle_actions(receiver : Receiver<actions::Action>) {
    loop {
        receiver.recv()
            .or(error::Error::ActionReceiveFailed.as_result::<actions::Action>())
            .and_then(actions::run)
            .unwrap_or_else(|e| println!("{}", e));
    }
}
