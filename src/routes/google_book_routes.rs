use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{
    handlers::google_book_handler::GoogleBookHandler, routes::protect_routes,
    services::google_book_service::GoogleBookServiceImpl,
};

pub fn google_book_routes() -> Router {
    // Serviço do Google Books
    let book_service = Arc::new(GoogleBookServiceImpl::new());

    // Handler
    let book_handler = Arc::new(GoogleBookHandler::new(book_service));
    let handler_clone = book_handler.clone();

    // Configurar rota protegida
    protect_routes(Router::new().route(
        "/api/books/search",
        post(move |body| async move { handler_clone.search_books(body).await }),
    ))
}
