use std::sync::mpsc::Receiver;
use std::collections::HashMap;

mod tcp;
mod dto;
mod actions;
mod error;
mod state;

use state::State;

fn main() {
    tcp::start_server()
        .map(handle_actions)
        .unwrap_or_else(|e| println!("{}", e));
}

fn handle_actions(receiver : Receiver<actions::Action>) {
    let mut state = State {
        clients: HashMap::new(),
    };

    loop {
        state = receiver.recv()
            .or(error::Error::ActionReceiveFailed.as_result::<actions::Action>())
            .and_then(|action| actions::run(action, state))
            .unwrap();
//            .unwrap_or_else(|e| {
//                println!("{}", e);
//                state
//            });
    }
}
