use devtime_infrastructure::sqlite::sqlite_version;
use serde::Serialize;
use tauri::State;

use crate::commands::CommandError;
use crate::state::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthDto {
    pub app_version: String,
    pub db_path: String,
    pub schema_version: i64,
    pub sqlite_version: String,
    pub started_at_ms: i64,
    pub now_ms: i64,
}

#[tauri::command]
pub fn health_check(state: State<'_, AppState>) -> Result<HealthDto, CommandError> {
    let schema_version = state
        .db
        .schema_version()
        .map_err(|error| CommandError::new("DATABASE_ERROR", error.to_string()))?;

    Ok(HealthDto {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        db_path: state.db.path().display().to_string(),
        schema_version,
        sqlite_version: sqlite_version(),
        started_at_ms: state.started_at_ms,
        now_ms: state.clock.now_ms(),
    })
}
