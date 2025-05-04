use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::book_service::BookService;

pub struct BookHandler {
    book_service: Arc<dyn BookService>,
}

impl BookHandler {
    pub fn new(book_service: Arc<dyn BookService>) -> Self {
        Self {
            book_service,
        }
    }

    pub async fn get_user_books(
        &self,
        Extension(user_id): Extension<Uuid>,
    ) -> Result<impl IntoResponse, AppError> {
        let user_books = self
            .book_service
            .get_user_books(&user_id)
            .await?;

        Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Livros do usuário recuperados com sucesso",
                "data": user_books
            })),
        ))
    }
} 