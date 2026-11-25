use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use devtime_application::errors::PreferenceError;
use devtime_application::ports::Preferences;
use rusqlite::{params, Connection, OptionalExtension};
use thiserror::Error;

const INIT_SQL: &str = include_str!("../../../../db/migrations/0001_init.sql");
const SCHEMA_VERSION: i64 = 1;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("no se pudo preparar el directorio de datos: {0}")]
    Io(#[from] std::io::Error),
    #[error("error de SQLite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("el bloqueo de la base de datos fue envenenado")]
    Poisoned,
}

pub struct Database {
    path: PathBuf,
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&path)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.pragma_update(None, "auto_vacuum", "INCREMENTAL")?;

        let database = Self {
            path,
            conn: Mutex::new(conn),
        };
        database.migrate()?;
        Ok(database)
    }

    fn lock(&self) -> Result<MutexGuard<'_, Connection>, DatabaseError> {
        self.conn.lock().map_err(|_| DatabaseError::Poisoned)
    }

    pub(crate) fn connection(&self) -> Result<MutexGuard<'_, Connection>, DatabaseError> {
        self.lock()
    }

    fn migrate(&self) -> Result<(), DatabaseError> {
        let conn = self.lock()?;
        let version: i64 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
        if version >= SCHEMA_VERSION {
            return Ok(());
        }

        conn.execute_batch(INIT_SQL)?;
        conn.execute(
            "INSERT OR REPLACE INTO schema_migrations (version, applied_at, checksum)
             VALUES (?1, ?2, ?3)",
            params![SCHEMA_VERSION, now_ms(), checksum(INIT_SQL.as_bytes())],
        )?;
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn schema_version(&self) -> Result<i64, DatabaseError> {
        let conn = self.lock()?;
        let version = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
        Ok(version)
    }

    pub fn table_exists(&self, table: &str) -> Result<bool, DatabaseError> {
        let conn = self.lock()?;
        let found: Option<i64> = conn
            .query_row(
                "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1",
                [table],
                |row| row.get(0),
            )
            .optional()?;
        Ok(found.is_some())
    }
}

impl Preferences for Database {
    fn get(&self, key: &str) -> Result<Option<String>, PreferenceError> {
        let conn = self.lock().map_err(|_| PreferenceError::Unavailable)?;
        let mut statement = conn
            .prepare("SELECT value_json FROM user_preferences WHERE key = ?1")
            .map_err(|error| PreferenceError::Read(error.to_string()))?;
        let value = statement
            .query_row([key], |row| row.get(0))
            .optional()
            .map_err(|error| PreferenceError::Read(error.to_string()))?;
        Ok(value)
    }

    fn set(&self, key: &str, value_json: &str) -> Result<(), PreferenceError> {
        let conn = self.lock().map_err(|_| PreferenceError::Unavailable)?;
        conn.execute(
            "INSERT INTO user_preferences (key, value_json, updated_at)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET
                 value_json = excluded.value_json,
                 updated_at = excluded.updated_at",
            params![key, value_json, now_ms()],
        )
        .map_err(|error| PreferenceError::Write(error.to_string()))?;
        Ok(())
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}

fn checksum(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use devtime_application::ports::Preferences;

    #[test]
    fn creates_schema_and_reports_version() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(dir.path().join("devtime.db")).unwrap();
        assert_eq!(db.schema_version().unwrap(), SCHEMA_VERSION);
        for table in [
            "projects",
            "tasks",
            "time_entries",
            "user_preferences",
            "activity_history",
            "sync_outbox",
        ] {
            assert!(db.table_exists(table).unwrap(), "falta la tabla {table}");
        }
    }

    #[test]
    fn reopening_database_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("devtime.db");
        drop(Database::open(&path).unwrap());
        let db = Database::open(&path).unwrap();
        assert_eq!(db.schema_version().unwrap(), SCHEMA_VERSION);
    }

    #[test]
    fn preferences_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(dir.path().join("devtime.db")).unwrap();
        assert_eq!(db.get("appearance.theme").unwrap(), None);
        db.set("appearance.theme", "\"dark\"").unwrap();
        db.set("appearance.theme", "\"light\"").unwrap();
        assert_eq!(
            db.get("appearance.theme").unwrap(),
            Some("\"light\"".to_string())
        );
    }
}
