use std::sync::Arc;

use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::config::Config;
use crate::handlers::auth_handler::AuthHandler;
use crate::repositories::user_repository::PgUserRepository;
use crate::services::auth_service::AuthServiceImpl;
use crate::services::password_service::create_password_service;

pub fn auth_routes(pool: Arc<PgPool>) -> Router {
    let config = Config::from_env().expect("Falha ao carregar configuração");

    let user_repository = Arc::new(PgUserRepository::new(pool.as_ref().clone()));
    let password_service = create_password_service();

    let auth_service = Arc::new(AuthServiceImpl::new(
        user_repository,
        password_service,
        config,
    ));

    let auth_handler = Arc::new(AuthHandler::new(auth_service));

    let handler_clone = auth_handler.clone();

    Router::new()
        .route(
            "/api/auth/register",
            post(move |body| async move { handler_clone.register(body).await }),
        )
        .route(
            "/api/auth/login",
            post(move |body| async move { auth_handler.login(body).await }),
        )
}
