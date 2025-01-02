use crate::user::User;
use sqlx::SqlitePool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("Record not found")]
    NotFound,

    #[error("Duplicate key: {0}")]
    UniqueViolation(&'static str),

    #[error("Invalid field: {0}")]
    InvalidField(&'static str),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

fn map_errors(err: sqlx::Error) -> UserRepositoryError {
    match &err {
        sqlx::Error::Database(db_err) => {
            if db_err.is_unique_violation() {
                UserRepositoryError::UniqueViolation("Duplicate key violation")
            } else {
                // todo: a workaround
                let error_msg = db_err.message();
                if error_msg.contains("NOT NULL") {
                    UserRepositoryError::InvalidField("Required field is missing")
                } else {
                    UserRepositoryError::DatabaseError(err)
                }
            }
        }
        sqlx::Error::RowNotFound => UserRepositoryError::NotFound,
        _ => UserRepositoryError::DatabaseError(err),
    }
}

#[derive(Clone)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<User>, UserRepositoryError> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
            .map_err(map_errors)
    }
}
