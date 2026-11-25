use devtime_domain::TimeEntry;

use crate::errors::ApplicationError;
use crate::ports::{ActiveEntry, DailyTotal, TimeEntries};

pub const DEFAULT_PROJECT_ID: &str = "project-general";
pub const DEFAULT_TASK_ID: &str = "task-general";

/// Dos latidos perdidos (60 s cada uno) antes de considerar una entrada huérfana.
pub const RECOVERY_STALE_AFTER_MS: i64 = 120_000;

fn require_current<E: TimeEntries>(repo: &E) -> Result<ActiveEntry, ApplicationError> {
    repo.find_current()?.ok_or(ApplicationError::NoActiveEntry)
}

pub fn current<E: TimeEntries>(repo: &E) -> Result<Option<ActiveEntry>, ApplicationError> {
    Ok(repo.find_current()?)
}

pub fn start<E: TimeEntries>(
    repo: &E,
    now_ms: i64,
    task_id: &str,
    new_id: &str,
) -> Result<ActiveEntry, ApplicationError> {
    let next = TimeEntry::start(new_id, task_id, now_ms);
    let previous = repo.find_current()?;
    let closed = match previous {
        Some(mut active) => {
            active.entry.close(now_ms)?;
            Some(active.entry)
        }
        None => None,
    };

    repo.switch(closed.as_ref(), &next)?;
    repo.find_current()?.ok_or(ApplicationError::NoActiveEntry)
}

pub fn pause<E: TimeEntries>(repo: &E, now_ms: i64) -> Result<ActiveEntry, ApplicationError> {
    let mut active = require_current(repo)?;
    active.entry.pause(now_ms)?;
    repo.save(&active.entry)?;
    Ok(active)
}

pub fn resume<E: TimeEntries>(repo: &E, now_ms: i64) -> Result<ActiveEntry, ApplicationError> {
    let mut active = require_current(repo)?;
    active.entry.resume(now_ms)?;
    repo.save(&active.entry)?;
    Ok(active)
}

pub fn close<E: TimeEntries>(repo: &E, now_ms: i64) -> Result<(), ApplicationError> {
    let mut active = require_current(repo)?;
    active.entry.close(now_ms)?;
    repo.save(&active.entry)?;
    Ok(())
}

/// Pausa una entrada en curso cuyo último latido quedó atrás.
pub fn recover_stale<E: TimeEntries>(
    repo: &E,
    now_ms: i64,
    stale_after_ms: i64,
) -> Result<bool, ApplicationError> {
    let Some(mut active) = repo.find_current()? else {
        return Ok(false);
    };

    if !active.entry.recover_if_stale(now_ms, stale_after_ms)? {
        return Ok(false);
    }

    repo.save(&active.entry)?;
    Ok(true)
}

pub fn today_totals<E: TimeEntries>(
    repo: &E,
    now_ms: i64,
) -> Result<Vec<DailyTotal>, ApplicationError> {
    Ok(repo.daily_totals(now_ms)?)
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use devtime_domain::{EntryStatus, TimeEntry};

    use super::*;
    use crate::errors::RepositoryError;

    const T0: i64 = 1_700_000_000_000;

    #[derive(Default)]
    struct FakeTimeEntries {
        entries: Mutex<Vec<TimeEntry>>,
    }

    impl FakeTimeEntries {
        fn active(entry: TimeEntry) -> ActiveEntry {
            ActiveEntry {
                entry,
                project_id: DEFAULT_PROJECT_ID.to_string(),
                project_name: "General".to_string(),
                project_code: "GEN".to_string(),
                project_color: "#2F81F7".to_string(),
                task_title: "Trabajo general".to_string(),
            }
        }

        fn snapshot(&self) -> Vec<TimeEntry> {
            self.entries.lock().unwrap().clone()
        }
    }

    impl TimeEntries for FakeTimeEntries {
        fn find_current(&self) -> Result<Option<ActiveEntry>, RepositoryError> {
            let entries = self.entries.lock().unwrap();
            Ok(entries
                .iter()
                .rev()
                .find(|entry| entry.status != EntryStatus::Closed)
                .cloned()
                .map(Self::active))
        }

        fn save(&self, entry: &TimeEntry) -> Result<(), RepositoryError> {
            let mut entries = self.entries.lock().unwrap();
            if let Some(slot) = entries.iter_mut().find(|item| item.id == entry.id) {
                *slot = entry.clone();
            }
            Ok(())
        }

        fn switch(
            &self,
            previous: Option<&TimeEntry>,
            next: &TimeEntry,
        ) -> Result<(), RepositoryError> {
            let mut entries = self.entries.lock().unwrap();
            if let Some(previous) = previous {
                if let Some(slot) = entries.iter_mut().find(|item| item.id == previous.id) {
                    *slot = previous.clone();
                }
            }
            entries.push(next.clone());
            Ok(())
        }

        fn daily_totals(&self, _now_ms: i64) -> Result<Vec<DailyTotal>, RepositoryError> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn start_creates_entry_when_idle() {
        let repo = FakeTimeEntries::default();
        let active = start(&repo, T0, DEFAULT_TASK_ID, "e1").unwrap();
        assert_eq!(active.entry.id, "e1");
        assert_eq!(active.entry.status, EntryStatus::Running);
    }

    #[test]
    fn start_closes_previous_entry() {
        let repo = FakeTimeEntries::default();
        start(&repo, T0, DEFAULT_TASK_ID, "e1").unwrap();
        start(&repo, T0 + 90_000, DEFAULT_TASK_ID, "e2").unwrap();

        let entries = repo.snapshot();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].status, EntryStatus::Closed);
        assert_eq!(entries[0].duration_seconds, Some(90));
        assert_eq!(entries[1].status, EntryStatus::Running);
    }

    #[test]
    fn pause_and_resume_update_same_entry() {
        let repo = FakeTimeEntries::default();
        start(&repo, T0, DEFAULT_TASK_ID, "e1").unwrap();

        let paused = pause(&repo, T0 + 30_000).unwrap();
        assert_eq!(paused.entry.status, EntryStatus::Paused);
        assert_eq!(paused.entry.accumulated_seconds, 30);

        let resumed = resume(&repo, T0 + 90_000).unwrap();
        assert_eq!(resumed.entry.status, EntryStatus::Running);
        assert_eq!(resumed.entry.last_resumed_at, Some(T0 + 90_000));
        assert_eq!(repo.snapshot().len(), 1);
    }

    #[test]
    fn close_removes_current_entry() {
        let repo = FakeTimeEntries::default();
        start(&repo, T0, DEFAULT_TASK_ID, "e1").unwrap();
        close(&repo, T0 + 45_000).unwrap();

        assert!(current(&repo).unwrap().is_none());
        let entries = repo.snapshot();
        assert_eq!(entries[0].status, EntryStatus::Closed);
        assert_eq!(entries[0].duration_seconds, Some(45));
    }

    #[test]
    fn actions_require_active_entry() {
        let repo = FakeTimeEntries::default();
        let error = pause(&repo, T0).unwrap_err();
        assert_eq!(error.code(), "NO_ACTIVE_ENTRY");
        assert!(close(&repo, T0).is_err());
    }

    #[test]
    fn recovery_pauses_stale_entry() {
        let repo = FakeTimeEntries::default();
        start(&repo, T0, DEFAULT_TASK_ID, "e1").unwrap();
        repo.entries
            .lock()
            .unwrap()
            .last_mut()
            .unwrap()
            .heartbeat(T0 + 10_000);

        assert!(recover_stale(&repo, T0 + 600_000, 120_000).unwrap());
        let active = current(&repo).unwrap().unwrap();
        assert_eq!(active.entry.status, EntryStatus::Paused);
        assert_eq!(active.entry.accumulated_seconds, 10);
    }
}
