use crate::user::{UserService, UserServiceError};
use actix_web::{web, HttpResponse, Responder};

fn map_errors(err: UserServiceError) -> HttpResponse {
    match err {
        UserServiceError::NotFound => HttpResponse::NotFound().body("User not found"),
        UserServiceError::ConflictError(msg) => HttpResponse::Conflict().body(msg),
        UserServiceError::ValidationError(msg) => HttpResponse::BadRequest().body(msg),
        UserServiceError::DatabaseError(_) => {
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}

pub async fn list_users(service: web::Data<UserService>) -> impl Responder {
    match service.list_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => map_errors(e),
    }
}
