use std::sync::Arc;

use axum::{
    routing::{delete, post},
    Router,
};
use sqlx::PgPool;

use crate::{
    handlers::book_offered_handler::BookOfferedHandler,
    repositories::{
        book_repository::PgBookRepository, 
        books_offered_repository::PgBooksOfferedRepository
    },
    routes::protect_routes,
    services::{
        book_offered_service::BookOfferedServiceImpl,
        google_book_service::GoogleBookServiceImpl,
        http_service::HttpServiceImpl,
    },
};

pub fn book_offered_routes(pool: Arc<PgPool>) -> Router {
    // Repositórios
    let book_repository = Arc::new(PgBookRepository::new(pool.as_ref().clone()));
    let books_offered_repository = Arc::new(PgBooksOfferedRepository::new(pool.as_ref().clone()));
    
    // Serviço HTTP
    let http_service = Arc::new(HttpServiceImpl::new());
    
    // Serviço do Google Books
    let google_book_service = Arc::new(GoogleBookServiceImpl::new(http_service));
    
    // Serviço de Livros Oferecidos
    let book_offered_service = Arc::new(BookOfferedServiceImpl::new(
        book_repository,
        books_offered_repository,
        google_book_service,
    ));

    // Handler
    let book_offered_handler = Arc::new(BookOfferedHandler::new(book_offered_service));
    let handler_clone = book_offered_handler.clone();
    let handler_clone2 = book_offered_handler.clone();

    // Configurar rotas protegidas
    protect_routes(
        Router::new()
            .route(
                "/api/books/offered",
                post(move |user_id, body| async move {
                    handler_clone.add_book_to_offered(user_id, body).await
                }),
            )
            .route(
                "/api/books/offered/:book_id",
                delete(move |user_id, path| async move {
                    handler_clone2.remove_book_from_offered(user_id, path).await
                }),
            ),
    )
} 