pub mod app;
mod config;
mod docs;
mod error;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use crate::config::Config;
use std::net::{SocketAddr, TcpListener};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
#[allow(dead_code)]
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

    let app = app::create_app(&config.database_url).await;

    // Configura o listener na porta especificada
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], config.port)))
        .expect("Falha ao vincular à porta");

    // Iniciar servidor
    tracing::info!("Servidor iniciado em {}", listener.local_addr().unwrap());

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
