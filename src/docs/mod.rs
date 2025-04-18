pub mod auth_docs;
pub mod google_book_docs;

use crate::models::book::{BookSearchRequest, GoogleBookDto};
use crate::models::user::{CreateUserDto, LoginUserDto, TokenResponse, UserResponse};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::docs::auth_docs::register,
        crate::docs::auth_docs::login,
        crate::docs::google_book_docs::search_books,
    ),
    components(
        schemas(CreateUserDto, LoginUserDto, TokenResponse, UserResponse, BookSearchRequest, GoogleBookDto)
    ),
    tags(
        (name = "auth", description = "API de autenticação"),
        (name = "google_books", description = "API de livros do Google")
    ),
    info(
        title = "API Troca Livros",
        version = "1.0.0",
        description = "API para o sistema de troca de livros",
    )
)]
pub struct ApiDoc;
