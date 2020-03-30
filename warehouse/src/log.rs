extern crate rusqlite;

use rusqlite::{params, Connection, ToSql};
use rusqlite::types::{ToSqlOutput, Value};
use std::fmt;
use crate::error::Error;

#[derive(Copy, Clone, Debug)]
pub enum LogType {
    Log,
    Warning,
    Error,
}

impl LogType {
    pub fn from_string(log_type: String) -> Result<LogType, Error> {
        match log_type.as_str() {
            "log" => Ok(LogType::Log),
            "warning" => Ok(LogType::Warning),
            "error" => Ok(LogType::Error),
            _ => Error::failed_to_convert_log_type::<LogType>(),
        }
    }

    pub fn to_string(self) -> String {
        match self {
            LogType::Log => "log".to_string(),
            LogType::Warning => "warning".to_string(),
            LogType::Error => "error".to_string(),
        }
    }
}

impl ToSql for LogType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        let log_type = format!("{}", self);
        Ok(ToSqlOutput::Owned(Value::Text(log_type)))
    }
}

impl fmt::Display for LogType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Log {
    pub id: i64,
    pub log_type: LogType,
    pub message: String,
    pub stack_trace: String,
}

impl Log {
    pub fn connection() -> Result<Connection, Error> {
        let conn = Connection::open("../logs.db")
            .or_else(Error::rusqlite_error)?;

        conn.execute(
            "create table if not exists logs (
                id integer primary key,
                log_type text not null,
                message text not null,
                stack_trace text not null
            )",
            params![]
        ).or_else(Error::rusqlite_error)?;

        Ok(conn)
    }

    pub fn fetch(conn: &Connection) -> Result<Vec<Log>, Error> {
        conn.prepare("SELECT * FROM logs")
            .or_else(Error::rusqlite_error)?
            .query_map(params![], |row| {
                let id = row.get(0)?;
                let log_type = row.get(1)?;
                let message = row.get(2)?;
                let stack_trace = row.get(3)?;

                Ok((id, log_type, message, stack_trace))
            })
            .or_else(Error::rusqlite_error)?
            .map(|result| {
                let (id, log_type, message, stack_trace) = result
                    .or_else(Error::rusqlite_error)?;
                let converted_log_type = LogType::from_string(log_type)?;

                Ok(Log {
                    id: id,
                    log_type: converted_log_type,
                    message: message,
                    stack_trace: stack_trace,
                })
            })
            .collect()
    }

    pub fn persist(&self, conn: &Connection) -> Result<Log, Error> {
        conn.execute(
            "INSERT INTO logs (log_type, message, stack_trace) values (?1, ?2, ?3)",
            params![self.log_type, self.message, self.stack_trace]
        ).or_else(Error::rusqlite_error)?;

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
