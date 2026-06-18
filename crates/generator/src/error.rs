use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Template error: {0}")]
    Template(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Generation error: {0}")]
    Generation(String),
}

pub type Result<T> = std::result::Result<T, Error>;