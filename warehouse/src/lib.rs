pub mod log;
pub mod error;

pub fn main() -> Result<()> {
    Ok(())
}

pub type Result<T> = rusqlite::Result<T>;
