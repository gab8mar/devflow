mod database;
mod time_entries;

pub use database::{Database, DatabaseError};

pub fn sqlite_version() -> String {
    rusqlite::version().to_string()
}
