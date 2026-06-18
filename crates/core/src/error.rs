use thiserror::Error;

#[derive(Debug, Error)]
pub enum DevaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Auth error: {0}")]
    Auth(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("GitHub error: {0}")]
    GitHub(String),

    #[error("Azure DevOps error: {0}")]
    AzureDevOps(String),

    #[error("Telegram error: {0}")]
    Telegram(String),

    #[error("Scaffolding error: {0}")]
    Scaffolding(String),
}