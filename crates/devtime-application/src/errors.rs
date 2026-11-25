use devtime_domain::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PreferenceError {
    #[error("no se pudo leer la preferencia: {0}")]
    Read(String),
    #[error("no se pudo guardar la preferencia: {0}")]
    Write(String),
    #[error("el almacén de preferencias no está disponible")]
    Unavailable,
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("error de base de datos: {0}")]
    Database(String),
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("clave de preferencia inválida: {0}")]
    InvalidKey(String),
    #[error(transparent)]
    Preferences(#[from] PreferenceError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("no hay una entrada de tiempo activa")]
    NoActiveEntry,
    #[error("JSON inválido: {0}")]
    Json(#[from] serde_json::Error),
}

impl ApplicationError {
    pub fn code(&self) -> &'static str {
        match self {
            ApplicationError::InvalidKey(_) => "INVALID_PREFERENCE_KEY",
            ApplicationError::Preferences(_) => "PREFERENCES_UNAVAILABLE",
            ApplicationError::Repository(_) => "REPOSITORY_ERROR",
            ApplicationError::Domain(error) => error.code(),
            ApplicationError::NoActiveEntry => "NO_ACTIVE_ENTRY",
            ApplicationError::Json(_) => "INVALID_JSON",
        }
    }
}
