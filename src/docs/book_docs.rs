#[allow(unused_imports)]
use crate::error::AppError;
#[allow(unused_imports)]
use crate::services::book_service::UserBooks;
use utoipa::{ToSchema};
#[allow(unused_imports)]
use uuid::Uuid;

#[derive(ToSchema)]
pub struct UserBooksResponse {
    pub status: String,
    pub message: String,
    pub data: UserBooks
}

/// Buscar livros do usuário (possuídos e desejados)
#[utoipa::path(
    get,
    path = "/api/books",
    tag = "books",
    responses(
        (status = 200, description = "Livros recuperados com sucesso", body = UserBooksResponse),
        (status = 401, description = "Não autorizado", body = AppError),
        (status = 500, description = "Erro interno do servidor", body = AppError),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub fn get_user_books() {} 