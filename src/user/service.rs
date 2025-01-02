use crate::user::{User, UserRepository, UserRepositoryError};
use clerk_rs::{apis::jwks_api::Jwks, clerk::Clerk, ClerkConfiguration};
use thiserror::Error;
use tracing::{error, info};

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

    #[error("Bad Gateway: External service error: {0}")]
    BadGateway(String),
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
    clerk: Clerk,
}

impl UserService {
    pub fn new(repository: UserRepository, config: ClerkConfiguration) -> Self {
        let clerk = Clerk::new(config);

        Self { repository, clerk }
    }

    pub async fn list_users(&self) -> Result<Vec<User>, UserServiceError> {
        self.repository.list().await.map_err(map_errors)
    }

    pub async fn login_or_register(&self) -> Result<String, UserServiceError> {
        let keys = match Jwks::get_jwks(&self.clerk).await {
            Ok(model) => model
                .keys
                .first()
                .cloned()
                .ok_or_else(|| UserServiceError::BadGateway("No keys found".to_string()))?,
            Err(err) => return Err(UserServiceError::BadGateway(err.to_string())),
        };
        info!("{:?}", keys);
        info!("{:?}", keys.n);

        // todo: to be continued

        Ok("ok".to_string())
    }
}
