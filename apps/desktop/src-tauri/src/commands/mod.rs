pub mod health;
pub mod preferences;
pub mod timer;

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandError {
    pub code: String,
    pub message: String,
}

impl CommandError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl From<devtime_application::ApplicationError> for CommandError {
    fn from(error: devtime_application::ApplicationError) -> Self {
        let code = error.code();
        CommandError::new(code, error.to_string())
    }
}
