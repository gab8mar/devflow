use devtime_application::ports::{ActiveEntry, DailyTotal};
use devtime_application::use_cases::timer::{self, DEFAULT_TASK_ID};
use devtime_domain::EntryStatus;
use serde::Serialize;
use tauri::State;
use uuid::Uuid;

use crate::commands::CommandError;
use crate::state::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveEntryDto {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub project_code: String,
    pub project_color: String,
    pub task_id: String,
    pub task_title: String,
    pub status: String,
    pub started_at: i64,
    pub last_resumed_at: Option<i64>,
    pub accumulated_seconds: i64,
    pub elapsed_seconds: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerSnapshotDto {
    pub active: Option<ActiveEntryDto>,
    pub idle_state: &'static str,
    pub server_now_ms: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyTotalDto {
    pub project_id: String,
    pub project_name: String,
    pub project_code: String,
    pub task_id: String,
    pub task_title: String,
    pub total_seconds: i64,
    pub entry_count: i64,
}

#[tauri::command]
pub fn get_timer_snapshot(state: State<'_, AppState>) -> Result<TimerSnapshotDto, CommandError> {
    snapshot(&state)
}

#[tauri::command]
pub fn start_task(
    state: State<'_, AppState>,
    task_id: Option<String>,
) -> Result<TimerSnapshotDto, CommandError> {
    let task_id = task_id
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_TASK_ID.to_string());
    let entry_id = Uuid::now_v7().to_string();

    timer::start(state.db.as_ref(), state.clock.now_ms(), &task_id, &entry_id)?;
    snapshot(&state)
}

#[tauri::command]
pub fn pause_current(state: State<'_, AppState>) -> Result<TimerSnapshotDto, CommandError> {
    timer::pause(state.db.as_ref(), state.clock.now_ms())?;
    snapshot(&state)
}

#[tauri::command]
pub fn resume_current(state: State<'_, AppState>) -> Result<TimerSnapshotDto, CommandError> {
    timer::resume(state.db.as_ref(), state.clock.now_ms())?;
    snapshot(&state)
}

#[tauri::command]
pub fn close_current(state: State<'_, AppState>) -> Result<TimerSnapshotDto, CommandError> {
    timer::close(state.db.as_ref(), state.clock.now_ms())?;
    snapshot(&state)
}

#[tauri::command]
pub fn get_today_totals(state: State<'_, AppState>) -> Result<Vec<DailyTotalDto>, CommandError> {
    let totals = timer::today_totals(state.db.as_ref(), state.clock.now_ms())?;
    Ok(totals.into_iter().map(daily_total_dto).collect())
}

fn snapshot(state: &AppState) -> Result<TimerSnapshotDto, CommandError> {
    let now_ms = state.clock.now_ms();
    let active = timer::current(state.db.as_ref())?;
    let idle_state = match active.as_ref().map(|item| item.entry.status) {
        Some(EntryStatus::Running) => "active",
        Some(EntryStatus::Paused) => "paused",
        _ => "idle",
    };

    Ok(TimerSnapshotDto {
        active: active.map(|item| active_entry_dto(item, now_ms)),
        idle_state,
        server_now_ms: now_ms,
    })
}

fn active_entry_dto(active: ActiveEntry, now_ms: i64) -> ActiveEntryDto {
    let elapsed_seconds = active.entry.elapsed_seconds(now_ms);
    ActiveEntryDto {
        id: active.entry.id,
        project_id: active.project_id,
        project_name: active.project_name,
        project_code: active.project_code,
        project_color: active.project_color,
        task_id: active.entry.task_id,
        task_title: active.task_title,
        status: active.entry.status.as_str().to_string(),
        started_at: active.entry.started_at,
        last_resumed_at: active.entry.last_resumed_at,
        accumulated_seconds: active.entry.accumulated_seconds,
        elapsed_seconds,
    }
}

fn daily_total_dto(total: DailyTotal) -> DailyTotalDto {
    DailyTotalDto {
        project_id: total.project_id,
        project_name: total.project_name,
        project_code: total.project_code,
        task_id: total.task_id,
        task_title: total.task_title,
        total_seconds: total.total_seconds,
        entry_count: total.entry_count,
    }
}
