use serde::Serialize;
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
}
