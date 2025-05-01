use std::sync::Arc;

use axum::{
    routing::{delete, post},
    Router,
};
use sqlx::PgPool;

use crate::{
    handlers::book_wanted_handler::BookWantedHandler,
    repositories::{
        book_repository::PgBookRepository, 
        books_wanted_repository::PgBooksWantedRepository,
        books_offered_repository::PgBooksOfferedRepository
    },
    routes::protect_routes,
    services::{
        book_wanted_service::BookWantedServiceImpl,
        google_book_service::GoogleBookServiceImpl,
        http_service::HttpServiceImpl,
    },
};

pub fn book_wanted_routes(pool: Arc<PgPool>) -> Router {
    // Repositórios
    let book_repository = Arc::new(PgBookRepository::new(pool.as_ref().clone()));
    let books_wanted_repository = Arc::new(PgBooksWantedRepository::new(pool.as_ref().clone()));
    let books_offered_repository = Arc::new(PgBooksOfferedRepository::new(pool.as_ref().clone()));
    
    // Serviço HTTP
    let http_service = Arc::new(HttpServiceImpl::new());
    
    // Serviço do Google Books
    let google_book_service = Arc::new(GoogleBookServiceImpl::new(http_service));
    
    // Serviço de Livros Desejados
    let book_wanted_service = Arc::new(BookWantedServiceImpl::new(
        book_repository,
        books_wanted_repository,
        books_offered_repository,
        google_book_service,
    ));

    // Handler
    let book_wanted_handler = Arc::new(BookWantedHandler::new(book_wanted_service));
    let handler_clone = book_wanted_handler.clone();
    let handler_clone2 = book_wanted_handler.clone();

    // Configurar rotas protegidas
    protect_routes(
        Router::new()
            .route(
                "/api/books/wanted",
                post(move |user_id, body| async move {
                    handler_clone.add_book_to_wanted(user_id, body).await
                }),
            )
            .route(
                "/api/books/wanted/:book_id",
                delete(move |user_id, path| async move {
                    handler_clone2.remove_book_from_wanted(user_id, path).await
                }),
            ),
    )
} 