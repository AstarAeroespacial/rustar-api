use sqlx::Error as SqlxError;
use std::fmt;

/// Error type for repository-level operations (database layer)
#[derive(Debug)]
pub enum RepositoryError {
    /// SQLx or database-level error
    Database(String),

    /// No rows were affected when updating/deleting
    NotFound(String),

    /// Data integrity or constraint violation (e.g. unique key)
    Conflict(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::Database(msg) => write!(f, "Database error: {}", msg),
            RepositoryError::NotFound(msg) => write!(f, "Not found: {}", msg),
            RepositoryError::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

impl From<SqlxError> for RepositoryError {
    fn from(err: SqlxError) -> Self {
        let msg = err.to_string();

        if msg.contains("duplicate key") || msg.contains("unique constraint") {
            RepositoryError::Conflict(msg)
        } else {
            RepositoryError::Database(msg)
        }
    }
}
