pub mod log;

pub fn main() -> Result<()> {
    Ok(())
}

pub type Result<T> = rusqlite::Result<T>;
