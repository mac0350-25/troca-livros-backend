use std::sync::Arc;
use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::{
    handlers::trade_handler::TradeHandler,
    repositories::trade_repository::PgTradeRepository,
    routes::protect_routes,
    services::trade_service::TradeServiceImpl,
};

pub fn trade_routes(pool: Arc<PgPool>) -> Router {
    // Repositório
    let trade_repository = Arc::new(PgTradeRepository::new(pool.as_ref().clone()));
    
    // Serviço
    let trade_service = Arc::new(TradeServiceImpl::new(trade_repository));
    
    // Handler
    let trade_handler = Arc::new(TradeHandler::new(trade_service));
    let handler_clone = trade_handler.clone();

    // Configurar rotas protegidas
    protect_routes(
        Router::new()
            .route(
                "/api/trades/possible",
                get(move |user_id| async move {
                    handler_clone.get_possible_trades(user_id).await
                }),
            ),
    )
} 