pub mod auth_routes;
pub mod book_offered_routes;
pub mod book_wanted_routes;
pub mod google_book_routes;

use axum::{middleware::from_fn, Router};

use crate::middleware::auth_middleware::auth_middleware;

/// Função auxiliar para aplicar o middleware de autenticação a qualquer rota
///
/// Esta função facilita a proteção de rotas, mantendo consistência na aplicação
/// do middleware de autenticação em todo o projeto.
pub fn protect_routes(router: Router) -> Router {
    router.layer(from_fn(auth_middleware))
}
