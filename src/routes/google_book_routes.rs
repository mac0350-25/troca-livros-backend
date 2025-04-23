use std::sync::Arc;

use axum::{routing::post, Router};

use crate::handlers::google_book_handler::GoogleBookHandler;
use crate::services::google_book_service::GoogleBookServiceImpl;

pub fn google_book_routes() -> Router {
    let book_service = Arc::new(GoogleBookServiceImpl::new());
    let book_handler = Arc::new(GoogleBookHandler::new(book_service));

    let handler_clone = book_handler.clone();

    Router::new().route(
        "/api/books/search",
        post(move |body| async move { handler_clone.search_books(body).await }),
    )
}
