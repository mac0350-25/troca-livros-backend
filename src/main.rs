mod config;
mod docs;
mod error;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use crate::config::Config;
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

    // Criar o servidor
    let (listener, app) = troca_livros_api::create_server(
        &config.database_url,
        config.port,
        true, // Habilitar Swagger UI
    )
    .await;

    // Iniciar servidor
    tracing::info!("Servidor iniciado em {}", listener.local_addr().unwrap());

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
