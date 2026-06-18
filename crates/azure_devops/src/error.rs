use thiserror::Error;

#[derive(Error, Debug)]
pub enum AzureError {
    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Not found: {0}")]
    NotFound(String),
}
