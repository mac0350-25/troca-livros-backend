use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};
use sqlx::PgPool;

use crate::{
    handlers::book_handler::BookHandler,
    repositories::{
        book_repository::PgBookRepository, 
        books_offered_repository::PgBooksOfferedRepository,
        books_wanted_repository::PgBooksWantedRepository
    },
    routes::protect_routes,
    services::book_service::BookServiceImpl,
};

pub fn book_routes(pool: Arc<PgPool>) -> Router {
    // Repositórios
    let book_repository = Arc::new(PgBookRepository::new(pool.as_ref().clone()));
    let books_offered_repository = Arc::new(PgBooksOfferedRepository::new(pool.as_ref().clone()));
    let books_wanted_repository = Arc::new(PgBooksWantedRepository::new(pool.as_ref().clone()));
    
    // Serviço de Livros
    let book_service = Arc::new(BookServiceImpl::new(
        book_repository,
        books_offered_repository,
        books_wanted_repository,
    ));

    // Handler
    let book_handler = Arc::new(BookHandler::new(book_service));
    let handler_clone = book_handler.clone();

    // Configurar rotas protegidas
    protect_routes(
        Router::new()
            .route(
                "/api/books",
                get(move |user_id| async move {
                    handler_clone.get_user_books(user_id).await
                }),
            )
    )
} 