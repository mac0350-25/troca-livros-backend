use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

use crate::error::AppError;
use crate::models::book::BookSearchRequest;
use crate::services::google_book_service::GoogleBookService;

pub struct GoogleBookHandler {
    google_book_service: Arc<dyn GoogleBookService>,
}

impl GoogleBookHandler {
    pub fn new(book_service: Arc<dyn GoogleBookService>) -> Self {
        Self {
            google_book_service: book_service,
        }
    }

    pub async fn search_books(
        &self,
        Json(search_request): Json<BookSearchRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        if search_request.query.is_empty() {
            return Err(AppError::ValidationError(
                "A consulta não pode estar vazia".to_string(),
            ));
        }

        let books = self
            .google_book_service
            .search_books(&search_request.query)
            .await?;

        Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Livros encontrados com sucesso",
                "data": books
            })),
        ))
    }
}
