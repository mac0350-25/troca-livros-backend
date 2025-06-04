use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::AppError, 
    services::trade_service::TradeService
};

/// Handler para operações relacionadas a trocas
pub struct TradeHandler {
    trade_service: Arc<dyn TradeService>,
}

impl TradeHandler {
    pub fn new(trade_service: Arc<dyn TradeService>) -> Self {
        Self { trade_service }
    }

    /// Busca possíveis trocas para o usuário autenticado
    pub async fn get_possible_trades(
        &self,
        Extension(user_id): Extension<Uuid>,
    ) -> Result<impl IntoResponse, AppError> {
        let trades = self.trade_service.find_possible_trades(user_id).await?;
        
        Ok((StatusCode::OK, Json(trades)))
    }
}

/// Função standalone para uso com axum routes
/// Busca possíveis trocas para o usuário autenticado
pub async fn get_possible_trades(
    Extension(user_id): Extension<Uuid>,
    Extension(trade_service): Extension<Arc<dyn TradeService>>,
) -> Result<impl IntoResponse, AppError> {
    let trades = trade_service.find_possible_trades(user_id).await?;
    
    Ok((StatusCode::OK, Json(trades)))
} 