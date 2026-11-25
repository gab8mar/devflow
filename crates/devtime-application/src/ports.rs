use devtime_domain::TimeEntry;

use crate::errors::{PreferenceError, RepositoryError};

pub trait Clock: Send + Sync {
    fn now_ms(&self) -> i64;
}

pub trait Preferences: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<String>, PreferenceError>;
    fn set(&self, key: &str, value_json: &str) -> Result<(), PreferenceError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveEntry {
    pub entry: TimeEntry,
    pub project_id: String,
    pub project_name: String,
    pub project_code: String,
    pub project_color: String,
    pub task_title: String,
}

impl ActiveEntry {
    pub fn elapsed_seconds(&self, now_ms: i64) -> i64 {
        self.entry.elapsed_seconds(now_ms)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyTotal {
    pub project_id: String,
    pub project_name: String,
    pub project_code: String,
    pub task_id: String,
    pub task_title: String,
    pub total_seconds: i64,
    pub entry_count: i64,
}

pub trait TimeEntries: Send + Sync {
    /// Entrada no cerrada más reciente (en curso o pausada).
    fn find_current(&self) -> Result<Option<ActiveEntry>, RepositoryError>;

    fn save(&self, entry: &TimeEntry) -> Result<(), RepositoryError>;

    /// Cierra la entrada anterior (si existe) e inserta la nueva en una sola transacción.
    fn switch(&self, previous: Option<&TimeEntry>, next: &TimeEntry)
        -> Result<(), RepositoryError>;

    /// Totales cerrados del día local que contiene `now_ms`.
    fn daily_totals(&self, now_ms: i64) -> Result<Vec<DailyTotal>, RepositoryError>;
}
