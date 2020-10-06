extern crate rusqlite;

use rusqlite::{params, Connection, ToSql, Row};
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

    pub fn as_string(self) -> String {
        match self {
            LogType::Log => "log".to_string(),
            LogType::Warning => "warning".to_string(),
            LogType::Error => "error".to_string(),
        }
    }
}

impl ToSql for LogType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        Ok(ToSqlOutput::Owned(Value::Text(self.as_string())))
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

    pub fn fetch(conn: &Connection) -> Result<Vec<Log>, Error> {
        conn.prepare("SELECT * FROM logs")?
            .query_map(params![], Log::unwrap_row)?
            .map(|values| values.unwrap())
            .map(Log::build_log)
            .collect()
    }

    pub fn fetch_with_limit(conn: &Connection, limit: i64) -> Result<Vec<Log>, Error> {
        if limit > 0 {
            conn.prepare("SELECT * FROM logs LIMIT (?)")?
                .query_map(params![limit], Log::unwrap_row)?
                .map(|values| values.unwrap())
                .map(Log::build_log)
                .collect()
        } else {
            Log::fetch(&conn)
        }
    }

    fn unwrap_row(row: &Row) -> Result<(i64, String, String, String), rusqlite::Error> {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    }

    fn build_log((id, log_type, message, stack_trace): (i64, String, String, String)) -> Result<Log, Error> {
        Ok(Log {
            id: id,
            log_type: LogType::from_string(log_type)?,
            message: message,
            stack_trace: stack_trace,
        })
    }

    pub fn persist(&self, conn: &Connection) -> Result<Log, Error> {
        conn.execute(
            "INSERT INTO logs (log_type, message, stack_trace) values (?1, ?2, ?3)",
            params![self.log_type, self.message, self.stack_trace]
        )?;

        Ok(Log {
            id: conn.last_insert_rowid(),
            log_type: self.log_type.clone(),
            message: self.message.clone(),
            stack_trace: self.stack_trace.clone(),
        })
    }

    pub fn truncate(conn: &Connection) -> Result<(), Error> {
        conn.execute("DELETE FROM logs", params![])?;
        Ok(())
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
