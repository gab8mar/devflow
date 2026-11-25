use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryStatus {
    Running,
    Paused,
    Closed,
}

impl EntryStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            EntryStatus::Running => "running",
            EntryStatus::Paused => "paused",
            EntryStatus::Closed => "closed",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "running" => Some(EntryStatus::Running),
            "paused" => Some(EntryStatus::Paused),
            "closed" => Some(EntryStatus::Closed),
            _ => None,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("la entrada ya está cerrada")]
    AlreadyClosed,
    #[error("la entrada no está en curso")]
    NotRunning,
    #[error("la entrada no está pausada")]
    NotPaused,
    #[error("transición inválida desde el estado {from:?}")]
    InvalidTransition { from: EntryStatus },
}

impl DomainError {
    pub fn code(&self) -> &'static str {
        match self {
            DomainError::AlreadyClosed => "ENTRY_ALREADY_CLOSED",
            DomainError::NotRunning => "ENTRY_NOT_RUNNING",
            DomainError::NotPaused => "ENTRY_NOT_PAUSED",
            DomainError::InvalidTransition { .. } => "INVALID_TRANSITION",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: String,
    pub task_id: String,
    pub status: EntryStatus,
    pub started_at: i64,
    pub last_resumed_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub accumulated_seconds: i64,
    pub duration_seconds: Option<i64>,
    pub last_heartbeat_at: Option<i64>,
}

impl TimeEntry {
    pub fn start(id: impl Into<String>, task_id: impl Into<String>, now_ms: i64) -> Self {
        Self {
            id: id.into(),
            task_id: task_id.into(),
            status: EntryStatus::Running,
            started_at: now_ms,
            last_resumed_at: Some(now_ms),
            ended_at: None,
            accumulated_seconds: 0,
            duration_seconds: None,
            last_heartbeat_at: Some(now_ms),
        }
    }

    pub fn elapsed_seconds(&self, now_ms: i64) -> i64 {
        match self.status {
            EntryStatus::Running => {
                let resumed_at = self.last_resumed_at.unwrap_or(now_ms);
                self.accumulated_seconds + (now_ms - resumed_at).max(0) / 1000
            }
            EntryStatus::Paused => self.accumulated_seconds,
            EntryStatus::Closed => self.duration_seconds.unwrap_or(self.accumulated_seconds),
        }
    }

    pub fn pause(&mut self, now_ms: i64) -> Result<(), DomainError> {
        if self.status != EntryStatus::Running {
            return Err(DomainError::NotRunning);
        }
        self.accumulated_seconds = self.elapsed_seconds(now_ms);
        self.last_resumed_at = None;
        self.status = EntryStatus::Paused;
        self.last_heartbeat_at = Some(now_ms);
        Ok(())
    }

    pub fn pause_at(
        &mut self,
        active_until_ms: i64,
        detected_at_ms: i64,
    ) -> Result<(), DomainError> {
        if self.status != EntryStatus::Running {
            return Err(DomainError::NotRunning);
        }
        let cutoff = active_until_ms.clamp(
            self.last_resumed_at.unwrap_or(active_until_ms),
            detected_at_ms,
        );
        self.accumulated_seconds = self.elapsed_seconds(cutoff);
        self.last_resumed_at = None;
        self.status = EntryStatus::Paused;
        self.last_heartbeat_at = Some(detected_at_ms);
        Ok(())
    }

    pub fn resume(&mut self, now_ms: i64) -> Result<(), DomainError> {
        if self.status != EntryStatus::Paused {
            return Err(DomainError::NotPaused);
        }
        self.last_resumed_at = Some(now_ms);
        self.status = EntryStatus::Running;
        self.last_heartbeat_at = Some(now_ms);
        Ok(())
    }

    pub fn close(&mut self, now_ms: i64) -> Result<(), DomainError> {
        if self.status == EntryStatus::Closed {
            return Err(DomainError::AlreadyClosed);
        }
        if self.status == EntryStatus::Running {
            self.accumulated_seconds = self.elapsed_seconds(now_ms);
        }
        self.ended_at = Some(now_ms);
        self.duration_seconds = Some(self.accumulated_seconds);
        self.last_resumed_at = None;
        self.status = EntryStatus::Closed;
        self.last_heartbeat_at = Some(now_ms);
        Ok(())
    }

    pub fn heartbeat(&mut self, now_ms: i64) {
        if self.status == EntryStatus::Running {
            self.last_heartbeat_at = Some(now_ms);
        }
    }

    /// Pausa una entrada en curso si su último latido quedó atrás.
    ///
    /// Devuelve `true` si la entrada fue recuperada. El tiempo se acumula
    /// hasta el último latido conocido, no hasta `now_ms`, para no contar
    /// el periodo en que la aplicación estuvo caída.
    pub fn recover_if_stale(
        &mut self,
        now_ms: i64,
        stale_after_ms: i64,
    ) -> Result<bool, DomainError> {
        if self.status != EntryStatus::Running {
            return Ok(false);
        }

        let last_seen = self
            .last_heartbeat_at
            .or(self.last_resumed_at)
            .unwrap_or(self.started_at)
            .clamp(self.started_at, now_ms);

        if now_ms.saturating_sub(last_seen) <= stale_after_ms {
            return Ok(false);
        }

        self.pause_at(last_seen, now_ms)?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const T0: i64 = 1_700_000_000_000;

    fn started() -> TimeEntry {
        TimeEntry::start("entry-1", "task-1", T0)
    }

    #[test]
    fn elapsed_tracks_running_time() {
        let entry = started();
        assert_eq!(entry.elapsed_seconds(T0 + 65_000), 65);
    }

    #[test]
    fn pause_freezes_elapsed_time() {
        let mut entry = started();
        entry.pause(T0 + 30_000).unwrap();
        assert_eq!(entry.status, EntryStatus::Paused);
        assert_eq!(entry.elapsed_seconds(T0 + 300_000), 30);
    }

    #[test]
    fn resume_excludes_paused_time() {
        let mut entry = started();
        entry.pause(T0 + 30_000).unwrap();
        entry.resume(T0 + 90_000).unwrap();
        assert_eq!(entry.elapsed_seconds(T0 + 120_000), 60);
    }

    #[test]
    fn close_snapshots_duration() {
        let mut entry = started();
        entry.pause(T0 + 30_000).unwrap();
        entry.resume(T0 + 90_000).unwrap();
        entry.close(T0 + 150_000).unwrap();
        assert_eq!(entry.status, EntryStatus::Closed);
        assert_eq!(entry.duration_seconds, Some(90));
        assert_eq!(entry.elapsed_seconds(T0 + 900_000), 90);
    }

    #[test]
    fn close_paused_entry_keeps_accumulated_time() {
        let mut entry = started();
        entry.pause(T0 + 42_000).unwrap();
        entry.close(T0 + 600_000).unwrap();
        assert_eq!(entry.duration_seconds, Some(42));
    }

    #[test]
    fn double_close_is_rejected() {
        let mut entry = started();
        entry.close(T0 + 1_000).unwrap();
        assert_eq!(entry.close(T0 + 2_000), Err(DomainError::AlreadyClosed));
    }

    #[test]
    fn pause_backdates_to_last_activity() {
        let mut entry = started();
        entry.pause_at(T0 + 60_000, T0 + 900_000).unwrap();
        assert_eq!(entry.accumulated_seconds, 60);
        assert_eq!(entry.last_heartbeat_at, Some(T0 + 900_000));
    }

    #[test]
    fn clock_going_backwards_never_subtracts_time() {
        let entry = started();
        assert_eq!(entry.elapsed_seconds(T0 - 5_000), 0);
    }

    #[test]
    fn heartbeat_only_updates_running_entries() {
        let mut entry = started();
        entry.heartbeat(T0 + 60_000);
        assert_eq!(entry.last_heartbeat_at, Some(T0 + 60_000));
        entry.pause(T0 + 70_000).unwrap();
        entry.heartbeat(T0 + 600_000);
        assert_eq!(entry.last_heartbeat_at, Some(T0 + 70_000));
    }

    #[test]
    fn stale_running_entry_is_recovered_up_to_last_heartbeat() {
        let mut entry = started();
        entry.heartbeat(T0 + 120_000);
        let recovered = entry.recover_if_stale(T0 + 600_000, 120_000).unwrap();
        assert!(recovered);
        assert_eq!(entry.status, EntryStatus::Paused);
        assert_eq!(entry.accumulated_seconds, 120);
        assert_eq!(entry.last_heartbeat_at, Some(T0 + 600_000));
    }

    #[test]
    fn fresh_running_entry_is_not_recovered() {
        let mut entry = started();
        entry.heartbeat(T0 + 100_000);
        let recovered = entry.recover_if_stale(T0 + 180_000, 120_000).unwrap();
        assert!(!recovered);
        assert_eq!(entry.status, EntryStatus::Running);
    }

    #[test]
    fn recovery_ignores_paused_and_closed_entries() {
        let mut paused = started();
        paused.pause(T0 + 10_000).unwrap();
        assert!(!paused.recover_if_stale(T0 + 600_000, 120_000).unwrap());

        let mut closed = started();
        closed.close(T0 + 10_000).unwrap();
        assert!(!closed.recover_if_stale(T0 + 600_000, 120_000).unwrap());
    }

    #[test]
    fn status_strings_roundtrip() {
        for status in [
            EntryStatus::Running,
            EntryStatus::Paused,
            EntryStatus::Closed,
        ] {
            assert_eq!(EntryStatus::parse(status.as_str()), Some(status));
        }
        assert_eq!(EntryStatus::parse("desconocido"), None);
    }
}
