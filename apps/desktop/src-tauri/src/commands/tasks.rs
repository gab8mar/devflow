use devtime_application::ports::TaskSummary;
use devtime_application::use_cases::tasks;
use serde::Serialize;
use tauri::State;
use uuid::Uuid;

use crate::commands::CommandError;
use crate::state::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDto {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub project_code: String,
    pub project_color: String,
    pub title: String,
    pub external_id: Option<String>,
    pub last_used_at: Option<i64>,
    pub is_local: bool,
}

#[tauri::command]
pub fn list_tasks(state: State<'_, AppState>) -> Result<Vec<TaskDto>, CommandError> {
    let tasks = tasks::list(state.db.as_ref())?;
    Ok(tasks.into_iter().map(task_dto).collect())
}

#[tauri::command]
pub fn create_task(
    state: State<'_, AppState>,
    title: String,
    project_id: Option<String>,
) -> Result<TaskDto, CommandError> {
    let task_id = Uuid::now_v7().to_string();
    let task = tasks::create_local(
        state.db.as_ref(),
        state.clock.now_ms(),
        project_id.as_deref(),
        &title,
        &task_id,
    )?;
    Ok(task_dto(task))
}

fn task_dto(task: TaskSummary) -> TaskDto {
    let is_local = task.is_local();
    TaskDto {
        id: task.id,
        project_id: task.project_id,
        project_name: task.project_name,
        project_code: task.project_code,
        project_color: task.project_color,
        title: task.title,
        external_id: task.external_id,
        last_used_at: task.last_used_at,
        is_local,
    }
}
