use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;

use crate::error::AppError;
use crate::models::trade::PossibleTrade;
use crate::repositories::trade_repository::TradeRepository;

#[async_trait]
pub trait TradeService: Send + Sync {
    async fn find_possible_trades(&self, user_id: Uuid) -> Result<Vec<PossibleTrade>, AppError>;
}

pub struct TradeServiceImpl {
    trade_repository: Arc<dyn TradeRepository>,
}

impl TradeServiceImpl {
    pub fn new(trade_repository: Arc<dyn TradeRepository>) -> Self {
        Self { trade_repository }
    }
}

#[async_trait]
impl TradeService for TradeServiceImpl {
    /// Busca todas as trocas possíveis para um usuário
    /// 
    /// Retorna uma lista de trocas onde:
    /// - O usuário oferece um livro que outro usuário quer
    /// - O outro usuário oferece um livro que o usuário quer
    /// 
    /// # Arguments
    /// 
    /// * `user_id` - UUID do usuário para buscar trocas possíveis
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<PossibleTrade>, AppError>` - Lista de trocas possíveis ou erro
    async fn find_possible_trades(&self, user_id: Uuid) -> Result<Vec<PossibleTrade>, AppError> {
        self.trade_repository.find_possible_trades(user_id).await
    }
} 