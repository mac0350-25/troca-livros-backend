use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    Extension, Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::Config,
    docs::ApiDoc,
    repositories::user_repository::PgUserRepository,
    routes::{
        auth_routes::auth_routes, 
        book_offered_routes::book_offered_routes,
        book_routes::book_routes,
        book_wanted_routes::book_wanted_routes,
        google_book_routes::google_book_routes
    },
    services::{auth_service::AuthServiceImpl, password_service::create_password_service},
};

/// Configura e retorna o pool de conexão com o banco de dados
pub async fn create_database_pool(database_url: &str) -> Arc<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");

    Arc::new(pool)
}

/// Cria e configura o aplicativo Axum com todas as rotas
///
/// Esta função recebe a URL do banco de dados e configura o aplicativo
/// É usada tanto pela aplicação principal quanto pelos testes
pub async fn create_app(database_url: &str) -> Router {
    let pool = create_database_pool(database_url).await;
    let config = Config::from_env().expect("Falha ao carregar configuração");

    // Criar serviço de autenticação compartilhado para todas as rotas protegidas
    let user_repository = Arc::new(PgUserRepository::new(pool.as_ref().clone()));
    let password_service = create_password_service();

    let auth_service = Arc::new(AuthServiceImpl::new(
        user_repository,
        password_service,
        config,
    ));

    // Configurar CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_origin(Any);

    // Definir rotas públicas (sem autenticação)
    let public_routes = Router::new().merge(auth_routes(pool.clone()));

    // Definir rotas protegidas (com autenticação)
    let protected_routes = Router::new()
        .merge(google_book_routes())
        .merge(book_offered_routes(pool.clone()))
        .merge(book_wanted_routes(pool.clone()))
        .merge(book_routes(pool.clone()))
        .layer(Extension(auth_service));

    // Inicializar o router básico
    let mut app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors);

    let openapi = ApiDoc::openapi();
    app = app.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi));

    app
}
