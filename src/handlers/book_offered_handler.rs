use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::book_offered_service::BookOfferedService;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddBookRequest {
    pub google_id: String,
}

pub struct BookOfferedHandler {
    book_offered_service: Arc<dyn BookOfferedService>,
}

impl BookOfferedHandler {
    pub fn new(book_offered_service: Arc<dyn BookOfferedService>) -> Self {
        Self {
            book_offered_service,
        }
    }

    pub async fn add_book_to_offered(
        &self,
        Extension(user_id): Extension<Uuid>,
        Json(add_book_request): Json<AddBookRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let book_offered = self
            .book_offered_service
            .add_book_to_offered(&add_book_request.google_id, &user_id)
            .await?;

        Ok((
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "message": "Livro adicionado à lista de possuídos com sucesso",
                "data": book_offered
            })),
        ))
    }

    pub async fn remove_book_from_offered(
        &self,
        Extension(user_id): Extension<Uuid>,
        Path(book_id): Path<Uuid>,
    ) -> Result<impl IntoResponse, AppError> {
        self.book_offered_service
            .remove_book_from_offered(&book_id, &user_id)
            .await?;

        Ok((
            StatusCode::OK,
            Json(json!({
                    "status": "success",
                    "message": "Livro removido da lista de possuídos com sucesso"
            })),
        ))
    }
}
