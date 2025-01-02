use crate::user::{User, UserRepository, UserRepositoryError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("User not found")]
    NotFound,

    #[error("User Conflict: {0}")]
    ConflictError(&'static str),

    #[error("Database error: {0}")]
    DatabaseError(#[from] UserRepositoryError),

    #[error("Validation error: {0}")]
    ValidationError(&'static str),
    /*
    #[error("Permission denied")]
    PermissionDenied,
    */

    /*
    #[error("Cache error: {0}")]
    CacheError(&'static str),
    */
}

fn map_errors(err: UserRepositoryError) -> UserServiceError {
    match err {
        UserRepositoryError::UniqueViolation(_) => {
            UserServiceError::ConflictError("Username already exists")
        }
        UserRepositoryError::InvalidField(msg) => UserServiceError::ValidationError(msg),
        UserRepositoryError::NotFound => UserServiceError::NotFound,
        _ => UserServiceError::DatabaseError(err),
    }
}

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, UserServiceError> {
        self.repository.list().await.map_err(map_errors)
    }
}
