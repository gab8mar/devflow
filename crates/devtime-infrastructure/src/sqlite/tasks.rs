use devtime_application::errors::RepositoryError;
use devtime_application::ports::{TaskSummary, Tasks};
use rusqlite::{params, OptionalExtension, Row};

use super::database::Database;

const SELECT_TASK: &str = "
SELECT
    t.id, p.id, p.name, p.code, p.color, t.title, t.external_id,
    (SELECT MAX(te.started_at) FROM time_entries te WHERE te.task_id = t.id) AS last_used_at
FROM tasks t
JOIN projects p ON p.id = t.project_id
WHERE t.deleted_at IS NULL
  AND t.status = 'active'
  AND p.deleted_at IS NULL
  AND p.is_archived = 0";

impl Tasks for Database {
    fn list_active(&self) -> Result<Vec<TaskSummary>, RepositoryError> {
        let conn = self.connection().map_err(repository_error)?;
        let mut statement = conn
            .prepare(&format!(
                "{SELECT_TASK}
                 ORDER BY
                     CASE WHEN t.id = 'task-general' THEN 1 ELSE 0 END,
                     COALESCE(last_used_at, 0) DESC,
                     p.sort_order,
                     t.sort_order,
                     t.title"
            ))
            .map_err(repository_error)?;

        let rows = statement
            .query_map([], map_task)
            .map_err(repository_error)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(repository_error)
    }

    fn find_task(&self, id: &str) -> Result<Option<TaskSummary>, RepositoryError> {
        let conn = self.connection().map_err(repository_error)?;
        let mut statement = conn
            .prepare(&format!("{SELECT_TASK} AND t.id = ?1"))
            .map_err(repository_error)?;
        statement
            .query_row([id], map_task)
            .optional()
            .map_err(repository_error)
    }

    fn create_local(
        &self,
        id: &str,
        project_id: &str,
        title: &str,
        now_ms: i64,
    ) -> Result<TaskSummary, RepositoryError> {
        let conn = self.connection().map_err(repository_error)?;

        let existing: Option<String> = conn
            .query_row(
                "SELECT id FROM tasks
                 WHERE project_id = ?1
                   AND lower(title) = lower(?2)
                   AND deleted_at IS NULL
                 LIMIT 1",
                params![project_id, title],
                |row| row.get(0),
            )
            .optional()
            .map_err(repository_error)?;

        if let Some(existing) = existing {
            let mut statement = conn
                .prepare(&format!("{SELECT_TASK} AND t.id = ?1"))
                .map_err(repository_error)?;
            return statement
                .query_row([existing], map_task)
                .optional()
                .map_err(repository_error)?
                .ok_or_else(|| {
                    RepositoryError::Database("la tarea existente no se pudo leer".into())
                });
        }

        conn.execute(
            "INSERT INTO tasks
                (id, project_id, title, external_id, task_type, status,
                 estimate_minutes, sort_order, created_at, updated_at, deleted_at)
             VALUES (?1, ?2, ?3, NULL, 'other', 'active', NULL, 0, ?4, ?4, NULL)",
            params![id, project_id, title, now_ms],
        )
        .map_err(repository_error)?;

        let mut statement = conn
            .prepare(&format!("{SELECT_TASK} AND t.id = ?1"))
            .map_err(repository_error)?;
        statement
            .query_row([id], map_task)
            .map_err(repository_error)
    }
}

fn map_task(row: &Row<'_>) -> rusqlite::Result<TaskSummary> {
    Ok(TaskSummary {
        id: row.get(0)?,
        project_id: row.get(1)?,
        project_name: row.get(2)?,
        project_code: row.get(3)?,
        project_color: row.get(4)?,
        title: row.get(5)?,
        external_id: row.get(6)?,
        last_used_at: row.get(7)?,
    })
}

fn repository_error(error: impl std::fmt::Display) -> RepositoryError {
    RepositoryError::Database(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sqlite::Database;
    use devtime_application::ports::TimeEntries;
    use devtime_application::use_cases::timer::{self, DEFAULT_PROJECT_ID, DEFAULT_TASK_ID};

    const T0: i64 = 1_700_000_000_000;

    fn database() -> (tempfile::TempDir, Database) {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(dir.path().join("devtime.db")).unwrap();
        db.ensure_default_workspace(T0).unwrap();
        (dir, db)
    }

    #[test]
    fn lists_default_task() {
        let (_dir, db) = database();
        let tasks = db.list_active().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, DEFAULT_TASK_ID);
        assert!(tasks[0].is_local());
    }

    #[test]
    fn creates_local_task_and_finds_it() {
        let (_dir, db) = database();
        let task = db
            .create_local("task-1", DEFAULT_PROJECT_ID, "Revisar PR", T0 + 1_000)
            .unwrap();

        assert_eq!(task.id, "task-1");
        assert_eq!(task.title, "Revisar PR");
        assert_eq!(task.project_code, "GEN");
        assert!(task.is_local());
        assert_eq!(db.find_task("task-1").unwrap().unwrap(), task);
    }

    #[test]
    fn reuses_task_with_same_title() {
        let (_dir, db) = database();
        let first = db
            .create_local("task-1", DEFAULT_PROJECT_ID, "Revisar PR", T0)
            .unwrap();
        let second = db
            .create_local("task-2", DEFAULT_PROJECT_ID, "revisar pr", T0)
            .unwrap();

        assert_eq!(first.id, second.id);
        assert_eq!(db.list_active().unwrap().len(), 2);
    }

    #[test]
    fn new_task_can_be_tracked() {
        let (_dir, db) = database();
        let task = db
            .create_local("task-1", DEFAULT_PROJECT_ID, "Revisar PR", T0)
            .unwrap();
        timer::start(&db, T0, &task.id, "e1").unwrap();

        let active = db.find_current().unwrap().unwrap();
        assert_eq!(active.entry.task_id, "task-1");
        assert_eq!(active.task_title, "Revisar PR");
    }
}
