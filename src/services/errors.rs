use actix_web::{HttpResponse, ResponseError};
use sqlx::error::DatabaseError;
use std::fmt;

/// Error type for the service layer
#[derive(Debug)]
pub enum ServiceError {
    /// Input validation or user-side error (e.g. bad request body)
    BadRequest(String),

    /// Resource already exists or business conflict (e.g. duplicate name)
    Conflict(String),

    /// Resource not found
    NotFound(String),

    /// Database or unexpected internal error
    Internal(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ServiceError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ServiceError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ServiceError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ServiceError {}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::BadRequest(msg) => HttpResponse::BadRequest().body(msg.clone()),
            ServiceError::Conflict(msg) => HttpResponse::Conflict().body(msg.clone()),
            ServiceError::NotFound(msg) => HttpResponse::NotFound().body(msg.clone()),
            ServiceError::Internal(msg) => HttpResponse::InternalServerError().body(msg.clone()),
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => ServiceError::NotFound("Resource not found".into()),
            sqlx::Error::Database(db_err) => {
                let db_err: &(dyn DatabaseError) = db_err.as_ref();
                let msg = db_err.message().to_string();
                if msg.contains("unique constraint") || msg.contains("duplicate key") {
                    ServiceError::Conflict("Duplicate entry".into())
                } else {
                    ServiceError::Internal(msg)
                }
            }
            _ => ServiceError::Internal(err.to_string()),
        }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ServiceError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        ServiceError::Internal(err.to_string())
    }
}

impl From<crate::repository::errors::RepositoryError> for ServiceError {
    fn from(err: crate::repository::errors::RepositoryError) -> Self {
        match err {
            crate::repository::errors::RepositoryError::NotFound(msg) => {
                ServiceError::NotFound(msg)
            }
            crate::repository::errors::RepositoryError::Conflict(msg) => {
                ServiceError::Conflict(msg)
            }
            crate::repository::errors::RepositoryError::Database(msg) => {
                ServiceError::Internal(msg)
            }
        }
    }
}
