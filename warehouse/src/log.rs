extern crate rusqlite;

use rusqlite::{params, Connection, Result};
use std::fmt;

pub struct Log {
    pub id: i64,
    pub log_type: String,
    pub message: String,
    pub stack_trace: String,
}

impl Log {
    pub fn connection() -> Result<Connection> {
        let conn = Connection::open("../logs.db")?;

        conn.execute(
            "create table if not exists logs (
                id integer primary key,
                log_type text not null,
                message text not null,
                stack_trace text not null
            )",
            params![]
        )?;

        Ok(conn)
    }

    pub fn fetch(conn: Connection) -> Result<Vec<Log>> {
        conn.prepare("SELECT * FROM logs")?.query_map(params![], |row| {
            Ok(Log {
                id: row.get(0)?,
                log_type: row.get(1)?,
                message: row.get(2)?,
                stack_trace: row.get(3)?,
            })
        })?.collect()
    }

    pub fn persist(&self, conn: &Connection) -> Result<Log> {
        conn.execute(
            "INSERT INTO logs (log_type, message, stack_trace) values (?1, ?2, ?3)",
            &[&self.log_type, &self.message, &self.stack_trace]
        )?;

        Ok(Log {
            id: conn.last_insert_rowid(),
            log_type: self.log_type.clone(),
            message: self.message.clone(),
            stack_trace: self.stack_trace.clone(),
        })
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}\nlog_type: {}\nmessage: {}\nstack_trace: {}\n",
            self.id,
            self.log_type,
            self.message,
            self.stack_trace
        )
    }
}
