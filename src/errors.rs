use thiserror::Error;

#[derive(Error, Debug)]
pub enum WswError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Person not found: {0}")]
    NotFound(String),

    #[error("Multiple people found matching '{0}'. Use --id to specify:")]
    MultipleMatches(String),

    #[error("Invalid field format: {0}. Expected 'Key=Value'")]
    InvalidFieldFormat(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, WswError>;
