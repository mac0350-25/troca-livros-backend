use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Erro de autenticação: {0}")]
    AuthError(String),
    
    #[error("Erro de validação: {0}")]
    ValidationError(String),
    
    #[error("Erro de banco de dados: {0}")]
    DatabaseError(String),
    
    #[error("Recurso não encontrado: {0}")]
    NotFoundError(String),
    
    #[error("Erro interno do servidor: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::AuthError(message) => (StatusCode::UNAUTHORIZED, message),
            AppError::ValidationError(message) => (StatusCode::BAD_REQUEST, message),
            AppError::DatabaseError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::NotFoundError(message) => (StatusCode::NOT_FOUND, message),
            AppError::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "status": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => AppError::NotFoundError("Registro não encontrado".to_string()),
            _ => AppError::DatabaseError(error.to_string()),
        }
    }
} 