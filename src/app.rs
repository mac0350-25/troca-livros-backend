use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::docs::ApiDoc;
use crate::routes::auth_routes::auth_routes;
use crate::routes::google_book_routes::google_book_routes;

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

    // Configurar CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_origin(Any);

    // Inicializar o router básico
    let mut app = Router::new()
        .merge(auth_routes(pool))
        .merge(google_book_routes())
        .layer(cors);

    let openapi = ApiDoc::openapi();
    app = app.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi));

    app
}
