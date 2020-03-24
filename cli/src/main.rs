extern crate warehouse;

use structopt::StructOpt;
use warehouse::log::Log;
use warehouse::Result;

#[derive(StructOpt)]
struct Command {
    action: String,
    payload: i64,
}

fn main() {
    let args = Command::from_args();
    let logs = test().unwrap();

    for log in logs {
        println!("{}", log);
    }
}

fn test() -> Result<Vec<Log>> {
    let conn = Log::connection()?;

//    let log = Log {
//        id: 0,
//        log_type: String::from("warning"),
//        message: String::from("some warning"),
//        stack_trace: String::from("stack trace"),
//    };
//
//    let new_log = log.persist(conn)?;

    let logs = Log::fetch(conn)?;

    Ok(logs)
}
