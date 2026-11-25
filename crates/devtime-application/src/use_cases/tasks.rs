use crate::errors::ApplicationError;
use crate::ports::{TaskSummary, Tasks};

use super::timer::DEFAULT_PROJECT_ID;

const MAX_TITLE_CHARS: usize = 200;

pub fn list<E: Tasks>(repo: &E) -> Result<Vec<TaskSummary>, ApplicationError> {
    Ok(repo.list_active()?)
}

pub fn create_local<E: Tasks>(
    repo: &E,
    now_ms: i64,
    project_id: Option<&str>,
    title: &str,
    new_id: &str,
) -> Result<TaskSummary, ApplicationError> {
    let title = title.trim();
    if title.is_empty() || title.chars().count() > MAX_TITLE_CHARS {
        return Err(ApplicationError::InvalidTaskTitle);
    }

    let project_id = project_id
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_PROJECT_ID);

    Ok(repo.create_local(new_id, project_id, title, now_ms)?)
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::errors::RepositoryError;

    const T0: i64 = 1_700_000_000_000;

    #[derive(Default)]
    struct FakeTasks {
        tasks: Mutex<Vec<TaskSummary>>,
    }

    impl Tasks for FakeTasks {
        fn list_active(&self) -> Result<Vec<TaskSummary>, RepositoryError> {
            Ok(self.tasks.lock().unwrap().clone())
        }

        fn find_task(&self, id: &str) -> Result<Option<TaskSummary>, RepositoryError> {
            Ok(self
                .tasks
                .lock()
                .unwrap()
                .iter()
                .find(|task| task.id == id)
                .cloned())
        }

        fn create_local(
            &self,
            id: &str,
            project_id: &str,
            title: &str,
            _now_ms: i64,
        ) -> Result<TaskSummary, RepositoryError> {
            let mut tasks = self.tasks.lock().unwrap();
            if let Some(existing) = tasks.iter().find(|task| {
                task.project_id == project_id && task.title.eq_ignore_ascii_case(title)
            }) {
                return Ok(existing.clone());
            }

            let task = TaskSummary {
                id: id.to_string(),
                project_id: project_id.to_string(),
                project_name: "General".to_string(),
                project_code: "GEN".to_string(),
                project_color: "#2F81F7".to_string(),
                title: title.to_string(),
                external_id: None,
                last_used_at: None,
            };
            tasks.push(task.clone());
            Ok(task)
        }
    }

    #[test]
    fn creates_local_task_trimmed() {
        let repo = FakeTasks::default();
        let task = create_local(&repo, T0, None, "  Revisar PR  ", "t1").unwrap();
        assert_eq!(task.id, "t1");
        assert_eq!(task.title, "Revisar PR");
        assert_eq!(task.project_id, DEFAULT_PROJECT_ID);
        assert!(task.is_local());
    }

    #[test]
    fn reuses_task_with_same_title() {
        let repo = FakeTasks::default();
        let first = create_local(&repo, T0, None, "Revisar PR", "t1").unwrap();
        let second = create_local(&repo, T0, None, "revisar pr", "t2").unwrap();
        assert_eq!(first.id, second.id);
        assert_eq!(repo.list_active().unwrap().len(), 1);
    }

    #[test]
    fn rejects_empty_or_too_long_title() {
        let repo = FakeTasks::default();
        assert_eq!(
            create_local(&repo, T0, None, "   ", "t1")
                .unwrap_err()
                .code(),
            "INVALID_TASK_TITLE"
        );
        let long = "x".repeat(MAX_TITLE_CHARS + 1);
        assert_eq!(
            create_local(&repo, T0, None, &long, "t2")
                .unwrap_err()
                .code(),
            "INVALID_TASK_TITLE"
        );
    }
}
