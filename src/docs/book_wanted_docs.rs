#[allow(unused_imports)]
use crate::error::AppError;
#[allow(unused_imports)]
use crate::handlers::book_wanted_handler::AddBookRequest;
#[allow(unused_imports)]
use crate::models::book::BookWanted;
use utoipa::{ToSchema};
#[allow(unused_imports)]
use uuid::Uuid;

#[derive(ToSchema)]
pub struct BookWantedResponse {
    pub status: String,
    pub message: String,
    pub data: BookWanted
}

#[derive(ToSchema)]
pub struct SuccessMessage {
    pub status: String,
    pub message: String
}

/// Adicionar um livro à lista de possuídos
#[utoipa::path(
    post,
    path = "/api/books/wanted",
    tag = "books_wanted",
    request_body = AddBookRequest,
    responses(
        (status = 201, description = "Livro adicionado com sucesso", body = BookWantedResponse),
        (status = 400, description = "Erro de validação", body = AppError),
        (status = 401, description = "Não autorizado", body = AppError),
        (status = 404, description = "Livro não encontrado", body = AppError),
        (status = 500, description = "Erro interno do servidor", body = AppError),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub fn add_book_to_wanted() {}

/// Remover um livro da lista de possuídos
#[utoipa::path(
    delete,
    path = "/api/books/wanted/{book_id}",
    tag = "books_wanted",
    params(
        ("book_id" = Uuid, Path, description = "ID do livro a ser removido da lista")
    ),
    responses(
        (status = 200, description = "Livro removido com sucesso", body = SuccessMessage),
        (status = 400, description = "Erro de validação", body = AppError),
        (status = 401, description = "Não autorizado", body = AppError),
        (status = 404, description = "Livro não encontrado", body = AppError),
        (status = 500, description = "Erro interno do servidor", body = AppError),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub fn remove_book_from_wanted() {} 