use devtime_application::errors::RepositoryError;
use devtime_application::ports::{ActiveEntry, DailyTotal, TimeEntries};
use devtime_application::use_cases::timer::{DEFAULT_PROJECT_ID, DEFAULT_TASK_ID};
use devtime_domain::{EntryStatus, TimeEntry};
use rusqlite::types::Type;
use rusqlite::{params, Connection, OptionalExtension, Row};

use super::database::{Database, DatabaseError};

const INSERT_ENTRY: &str = "
INSERT INTO time_entries (
    id, task_id, status, started_at, last_resumed_at, ended_at,
    accumulated_seconds, duration_seconds, last_heartbeat_at,
    source, note, billable, sync_state, external_id, idempotency_key,
    created_at, updated_at
) VALUES (
    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9,
    'manual', NULL, 1, 'local', NULL, ?10, ?11, ?12
)";

const UPDATE_ENTRY: &str = "
UPDATE time_entries SET
    status = ?2,
    last_resumed_at = ?3,
    ended_at = ?4,
    accumulated_seconds = ?5,
    duration_seconds = ?6,
    last_heartbeat_at = ?7,
    updated_at = ?8
WHERE id = ?1";

const SELECT_CURRENT: &str = "
SELECT
    te.id, te.task_id, te.status, te.started_at, te.last_resumed_at,
    te.ended_at, te.accumulated_seconds, te.duration_seconds, te.last_heartbeat_at,
    p.id, p.name, p.code, p.color, t.title
FROM time_entries te
JOIN tasks t ON t.id = te.task_id
JOIN projects p ON p.id = t.project_id
WHERE te.status IN ('running', 'paused')
ORDER BY te.updated_at DESC, te.started_at DESC
LIMIT 1";

impl Database {
    pub fn ensure_default_workspace(&self, now_ms: i64) -> Result<(), DatabaseError> {
        let mut conn = self.connection()?;
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT OR IGNORE INTO projects
                (id, name, code, external_id, color, is_archived, sort_order,
                 created_at, updated_at, deleted_at)
             VALUES (?1, 'General', 'GEN', NULL, '#2F81F7', 0, 0, ?2, ?2, NULL)",
            params![DEFAULT_PROJECT_ID, now_ms],
        )?;
        tx.execute(
            "INSERT OR IGNORE INTO tasks
                (id, project_id, title, external_id, task_type, status,
                 estimate_minutes, sort_order, created_at, updated_at, deleted_at)
             VALUES (?1, ?2, 'Trabajo general', NULL, 'other', 'active',
                 NULL, 0, ?3, ?3, NULL)",
            params![DEFAULT_TASK_ID, DEFAULT_PROJECT_ID, now_ms],
        )?;
        tx.commit()?;
        Ok(())
    }
}

impl TimeEntries for Database {
    fn find_current(&self) -> Result<Option<ActiveEntry>, RepositoryError> {
        let conn = self.connection().map_err(repository_error)?;
        let mut statement = conn.prepare(SELECT_CURRENT).map_err(repository_error)?;
        statement
            .query_row([], map_active_entry)
            .optional()
            .map_err(repository_error)
    }

    fn save(&self, entry: &TimeEntry) -> Result<(), RepositoryError> {
        let conn = self.connection().map_err(repository_error)?;
        conn.execute(
            UPDATE_ENTRY,
            params![
                entry.id,
                entry.status.as_str(),
                entry.last_resumed_at,
                entry.ended_at,
                entry.accumulated_seconds,
                entry.duration_seconds,
                entry.last_heartbeat_at,
                entry.last_heartbeat_at.unwrap_or(entry.started_at),
            ],
        )
        .map_err(repository_error)?;
        Ok(())
    }

    fn switch(
        &self,
        previous: Option<&TimeEntry>,
        next: &TimeEntry,
    ) -> Result<(), RepositoryError> {
        let mut conn = self.connection().map_err(repository_error)?;
        let tx = conn.transaction().map_err(repository_error)?;

        if let Some(previous) = previous {
            write_entry(&tx, previous)?;
        }
        insert_entry(&tx, next)?;

        tx.commit().map_err(repository_error)
    }

    fn daily_totals(&self, now_ms: i64) -> Result<Vec<DailyTotal>, RepositoryError> {
        let conn = self.connection().map_err(repository_error)?;
        let mut statement = conn
            .prepare(
                "SELECT project_id, project_name, project_code, task_id, task_title,
                        total_seconds, entry_count
                 FROM v_daily_totals
                 WHERE local_day = date(?1 / 1000, 'unixepoch', 'localtime')
                 ORDER BY total_seconds DESC, project_name, task_title",
            )
            .map_err(repository_error)?;

        let rows = statement
            .query_map([now_ms], |row| {
                Ok(DailyTotal {
                    project_id: row.get(0)?,
                    project_name: row.get(1)?,
                    project_code: row.get(2)?,
                    task_id: row.get(3)?,
                    task_title: row.get(4)?,
                    total_seconds: row.get(5)?,
                    entry_count: row.get(6)?,
                })
            })
            .map_err(repository_error)?;

        rows.collect::<Result<Vec<_>, _>>()
            .map_err(repository_error)
    }
}

fn insert_entry(conn: &Connection, entry: &TimeEntry) -> Result<(), RepositoryError> {
    let updated_at = entry.last_heartbeat_at.unwrap_or(entry.started_at);
    conn.execute(
        INSERT_ENTRY,
        params![
            entry.id,
            entry.task_id,
            entry.status.as_str(),
            entry.started_at,
            entry.last_resumed_at,
            entry.ended_at,
            entry.accumulated_seconds,
            entry.duration_seconds,
            entry.last_heartbeat_at,
            entry.id,
            entry.started_at,
            updated_at,
        ],
    )
    .map_err(repository_error)?;
    Ok(())
}

fn write_entry(conn: &Connection, entry: &TimeEntry) -> Result<(), RepositoryError> {
    conn.execute(
        UPDATE_ENTRY,
        params![
            entry.id,
            entry.status.as_str(),
            entry.last_resumed_at,
            entry.ended_at,
            entry.accumulated_seconds,
            entry.duration_seconds,
            entry.last_heartbeat_at,
            entry.last_heartbeat_at.unwrap_or(entry.started_at),
        ],
    )
    .map_err(repository_error)?;
    Ok(())
}

fn map_active_entry(row: &Row<'_>) -> rusqlite::Result<ActiveEntry> {
    let raw_status: String = row.get(2)?;
    let status = EntryStatus::parse(&raw_status)
        .ok_or_else(|| rusqlite::Error::InvalidColumnType(2, "status".to_string(), Type::Text))?;

    Ok(ActiveEntry {
        entry: TimeEntry {
            id: row.get(0)?,
            task_id: row.get(1)?,
            status,
            started_at: row.get(3)?,
            last_resumed_at: row.get(4)?,
            ended_at: row.get(5)?,
            accumulated_seconds: row.get(6)?,
            duration_seconds: row.get(7)?,
            last_heartbeat_at: row.get(8)?,
        },
        project_id: row.get(9)?,
        project_name: row.get(10)?,
        project_code: row.get(11)?,
        project_color: row.get(12)?,
        task_title: row.get(13)?,
    })
}

fn repository_error(error: impl std::fmt::Display) -> RepositoryError {
    RepositoryError::Database(error.to_string())
}

#[cfg(test)]
mod tests {
    use devtime_application::ports::Clock;
    use devtime_application::use_cases::timer;
    use devtime_domain::EntryStatus;

    use super::*;

    const T0: i64 = 1_700_000_000_000;

    fn database() -> (tempfile::TempDir, Database) {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(dir.path().join("devtime.db")).unwrap();
        db.ensure_default_workspace(T0).unwrap();
        (dir, db)
    }

    #[test]
    fn default_workspace_is_idempotent() {
        let (_dir, db) = database();
        db.ensure_default_workspace(T0 + 1_000).unwrap();
        assert!(db.find_current().unwrap().is_none());
    }

    #[test]
    fn switching_closes_previous_entry_atomically() {
        let (_dir, db) = database();
        timer::start(&db, T0, DEFAULT_TASK_ID, "e1").unwrap();
        let active = timer::start(&db, T0 + 90_000, DEFAULT_TASK_ID, "e2").unwrap();

        assert_eq!(active.entry.id, "e2");
        assert_eq!(active.entry.status, EntryStatus::Running);
        assert_eq!(active.project_code, "GEN");
        assert_eq!(active.task_title, "Trabajo general");

        let totals = timer::today_totals(&db, T0 + 90_000).unwrap();
        assert_eq!(totals.len(), 1);
        assert_eq!(totals[0].entry_count, 1);
        assert_eq!(totals[0].total_seconds, 90);
    }

    #[test]
    fn pause_resume_and_close_persist_state() {
        let (_dir, db) = database();
        timer::start(&db, T0, DEFAULT_TASK_ID, "e1").unwrap();
        timer::pause(&db, T0 + 30_000).unwrap();

        let paused = db.find_current().unwrap().unwrap();
        assert_eq!(paused.entry.status, EntryStatus::Paused);
        assert_eq!(paused.entry.accumulated_seconds, 30);

        timer::resume(&db, T0 + 60_000).unwrap();
        let running = db.find_current().unwrap().unwrap();
        assert_eq!(running.entry.status, EntryStatus::Running);
        assert_eq!(running.entry.last_resumed_at, Some(T0 + 60_000));
    }

    #[test]
    fn closed_entries_count_in_daily_totals() {
        let (_dir, db) = database();
        let now = crate::SystemClock.now_ms();
        timer::start(&db, now - 120_000, DEFAULT_TASK_ID, "e1").unwrap();
        timer::close(&db, now).unwrap();

        assert!(db.find_current().unwrap().is_none());
        let totals = timer::today_totals(&db, now).unwrap();
        assert_eq!(totals.len(), 1);
        assert_eq!(totals[0].total_seconds, 120);
    }

    #[test]
    fn stale_entries_are_recovered_as_paused() {
        let (_dir, db) = database();
        timer::start(&db, T0, DEFAULT_TASK_ID, "e1").unwrap();
        let mut active = db.find_current().unwrap().unwrap();
        active.entry.heartbeat(T0 + 10_000);
        db.save(&active.entry).unwrap();

        assert!(timer::recover_stale(&db, T0 + 600_000, 120_000).unwrap());
        let recovered = db.find_current().unwrap().unwrap();
        assert_eq!(recovered.entry.status, EntryStatus::Paused);
        assert_eq!(recovered.entry.accumulated_seconds, 10);
    }
}
