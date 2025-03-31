use std::sync::Arc;

use axum::{
    routing:: post,
    Router,
};
use sqlx::PgPool;

use crate::config::Config;
use crate::handlers::auth_handler::AuthHandler;
use crate::repositories::user_repository::PgUserRepository;
use crate::services::auth_service::AuthServiceImpl;

pub fn auth_routes(pool: Arc<PgPool>) -> Router {
    let config = Config::from_env().expect("Falha ao carregar configuração");
    
    let user_repository = Arc::new(PgUserRepository::new(pool.as_ref().clone()));
    let auth_service = Arc::new(AuthServiceImpl::new(user_repository, config));
    let auth_handler = Arc::new(AuthHandler::new(auth_service));
    
    let handler_clone = auth_handler.clone();
    
    Router::new()
        .route(
            "/api/auth/register",
            post(move |body| async move {
                handler_clone.register(body).await
            }),
        )
        .route(
            "/api/auth/login",
            post(move |body| async move {
                auth_handler.login(body).await
            }),
        )
} 