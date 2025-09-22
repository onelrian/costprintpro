use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Authentication error: {0}")]
    Unauthorized(String),
    
    #[error("Access denied: {0}")]
    Forbidden(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::DatabaseError(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::InternalServerError(msg) => {
                tracing::error!("Internal server error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}
