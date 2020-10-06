use std::sync::mpsc::Sender;
use std::collections::HashMap;

pub struct State {
    pub clients : HashMap<usize, Sender<String>>
}
