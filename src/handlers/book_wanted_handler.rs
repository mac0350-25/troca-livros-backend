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
use crate::services::book_wanted_service::BookWantedService;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddBookRequest {
    pub google_id: String,
}

pub struct BookWantedHandler {
    book_wanted_service: Arc<dyn BookWantedService>,
}

impl BookWantedHandler {
    pub fn new(book_wanted_service: Arc<dyn BookWantedService>) -> Self {
        Self {
            book_wanted_service,
        }
    }

    pub async fn add_book_to_wanted(
        &self,
        Extension(user_id): Extension<Uuid>,
        Json(add_book_request): Json<AddBookRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let book_wanted = self
            .book_wanted_service
            .add_book_to_wanted(&add_book_request.google_id, &user_id)
            .await?;

        Ok((
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "message": "Livro adicionado à lista de desejados com sucesso",
                "data": book_wanted
            })),
        ))
    }

    pub async fn remove_book_from_wanted(
        &self,
        Extension(user_id): Extension<Uuid>,
        Path(book_id): Path<Uuid>,
    ) -> Result<impl IntoResponse, AppError> {
        self.book_wanted_service
            .remove_book_from_wanted(&book_id, &user_id)
            .await?;

        Ok((
            StatusCode::OK,
            Json(json!({
                    "status": "success",
                    "message": "Livro removido da lista de desejados com sucesso"
            })),
        ))
    }
}
