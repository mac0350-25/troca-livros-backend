mod config;
mod error;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use crate::config::Config;
use crate::routes::auth_routes::auth_routes;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Carregar variáveis de ambiente
    dotenv::dotenv().ok();
    
    // Configurar logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Carregar configuração
    let config = Config::from_env().expect("Falha ao carregar configuração");
    
    // Configurar pool de conexão com o banco de dados
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");
    
    // Configurar CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_origin(Any);
    
    // Configurar rotas
    let app = auth_routes(Arc::new(pool)).layer(cors);
    
    // Iniciar servidor
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Servidor iniciado em {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
} 