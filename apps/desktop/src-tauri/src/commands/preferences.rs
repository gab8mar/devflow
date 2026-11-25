use devtime_application::use_cases::preferences;
use serde_json::Value;
use tauri::State;

use crate::commands::CommandError;
use crate::state::AppState;

#[tauri::command]
pub fn get_preference(
    state: State<'_, AppState>,
    key: String,
) -> Result<Option<String>, CommandError> {
    preferences::get_preference(state.db.as_ref(), &key).map_err(CommandError::from)
}

#[tauri::command]
pub fn set_preference(
    state: State<'_, AppState>,
    key: String,
    value_json: String,
) -> Result<(), CommandError> {
    let value: Value = serde_json::from_str(&value_json)
        .map_err(|error| CommandError::new("INVALID_JSON", error.to_string()))?;
    preferences::set_preference(state.db.as_ref(), &key, &value).map_err(CommandError::from)
}
