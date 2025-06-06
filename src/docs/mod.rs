pub mod auth_docs;
pub mod book_docs;
pub mod book_offered_docs;
pub mod google_book_docs;
pub mod book_wanted_docs;

use crate::handlers::book_offered_handler::AddBookRequest;
use crate::models::book::{BookOffered, BookSearchRequest, GoogleBookDto};
use crate::models::user::{CreateUserDto, LoginUserDto, TokenResponse, UserResponse};
use crate::docs::book_docs::UserBooksResponse;
use crate::docs::book_offered_docs::{BookOfferedResponse, SuccessMessage as OfferedSuccessMessage};
use crate::docs::book_wanted_docs::{BookWantedResponse, SuccessMessage as WantedSuccessMessage};
use crate::services::book_service::UserBooks;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

// Define um modificador para adicionar esquema de segurança
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Certifique-se de que o componente existe
        if openapi.components.is_none() {
            openapi.components = Some(utoipa::openapi::Components::new());
        }

        // Adicione o esquema de segurança
        let components = openapi.components.as_mut().unwrap();
        components.security_schemes.insert(
            "bearerAuth".to_string(),
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::docs::auth_docs::register,
        crate::docs::auth_docs::login,
        crate::docs::book_docs::get_user_books,
        crate::docs::google_book_docs::search_books,
        crate::docs::book_offered_docs::add_book_to_offered,
        crate::docs::book_offered_docs::remove_book_from_offered,
        crate::docs::book_wanted_docs::add_book_to_wanted,
        crate::docs::book_wanted_docs::remove_book_from_wanted,
    ),
    components(
        schemas(
            CreateUserDto, 
            LoginUserDto, 
            TokenResponse, 
            UserResponse, 
            BookSearchRequest, 
            GoogleBookDto, 
            BookOffered, 
            AddBookRequest,
            BookOfferedResponse,
            BookWantedResponse,
            UserBooksResponse,
            UserBooks,
            OfferedSuccessMessage,
            WantedSuccessMessage
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "API de autenticação"),
        (name = "books", description = "API de livros do usuário"),
        (name = "google_books", description = "API de livros do Google"),
        (name = "books_offered", description = "API de livros possuídos"),
        (name = "books_wanted", description = "API de livros desejados")
    ),
    info(
        title = "API Troca Livros",
        version = "1.0.0",
        description = "API para o sistema de troca de livros.\n\n\
                      **Autenticação**:\n\
                      A maioria dos endpoints requer autenticação usando Bearer Token.\n\
                      Para obter um token, faça login usando o endpoint `/api/auth/login`.\n\
                      Em seguida, inclua o token em todas as requisições no cabeçalho:\n\
                      `Authorization: Bearer <seu-token>`",
    ),
)]
pub struct ApiDoc;
