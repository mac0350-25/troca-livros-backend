use crate::{
    error::AppError,
    models::trade::PossibleTrade,
    services::trade_service::{TradeService, TradeServiceImpl},
    repositories::trade_repository::TradeRepository,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

// Mock do TradeRepository para testes
struct MockTradeRepository {
    should_fail: bool,
    mock_trades: Vec<PossibleTrade>,
}

impl MockTradeRepository {
    fn new(mock_trades: Vec<PossibleTrade>) -> Self {
        Self {
            should_fail: false,
            mock_trades,
        }
    }

    fn new_with_error() -> Self {
        Self {
            should_fail: true,
            mock_trades: vec![],
        }
    }
}

#[async_trait]
impl TradeRepository for MockTradeRepository {
    async fn find_possible_trades(&self, _user_id: Uuid) -> Result<Vec<PossibleTrade>, AppError> {
        if self.should_fail {
            Err(AppError::DatabaseError("Database connection failed".to_string()))
        } else {
            Ok(self.mock_trades.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{book::GoogleBookDto, user::UserResponse};
    use chrono::DateTime;

    fn create_mock_trade() -> PossibleTrade {
        let timestamp = DateTime::from_timestamp(1640995200, 0)
            .unwrap()
            .naive_utc();
            
        PossibleTrade {
            offered_book_id: Uuid::new_v4(),
            offered_book: GoogleBookDto {
                google_id: "test_book_1".to_string(),
                title: "Test Book 1".to_string(),
                authors: Some("Test Author 1".to_string()),
                publisher: Some("Test Publisher".to_string()),
                published_date: Some("2023-01-01".to_string()),
                description: Some("Test description 1".to_string()),
                image_url: Some("http://example.com/book1.jpg".to_string()),
                page_count: Some(200),
            },
            wanted_book_id: Uuid::new_v4(),
            wanted_book: GoogleBookDto {
                google_id: "test_book_2".to_string(),
                title: "Test Book 2".to_string(),
                authors: Some("Test Author 2".to_string()),
                publisher: Some("Test Publisher 2".to_string()),
                published_date: Some("2023-02-01".to_string()),
                description: Some("Test description 2".to_string()),
                image_url: Some("http://example.com/book2.jpg".to_string()),
                page_count: Some(300),
            },
            trade_partner: UserResponse {
                id: Uuid::new_v4(),
                name: "Test Partner".to_string(),
                email: "partner@test.com".to_string(),
                created_at: timestamp,
                updated_at: timestamp,
            },
        }
    }

    #[tokio::test]
    async fn test_find_possible_trades_success() {
        // Arrange
        let mock_trade = create_mock_trade();
        let mock_repository = Arc::new(MockTradeRepository::new(vec![mock_trade.clone()]));
        let trade_service = TradeServiceImpl::new(mock_repository);
        let user_id = Uuid::new_v4();

        // Act
        let result = trade_service.find_possible_trades(user_id).await;

        // Assert
        assert!(result.is_ok(), "Deve retornar trocas possíveis com sucesso");
        let trades = result.unwrap();
        assert_eq!(trades.len(), 1, "Deve retornar exatamente uma troca");
        assert_eq!(trades[0].offered_book.title, "Test Book 1", "Livro oferecido deve ser correto");
        assert_eq!(trades[0].wanted_book.title, "Test Book 2", "Livro desejado deve ser correto");
        assert_eq!(trades[0].trade_partner.name, "Test Partner", "Parceiro deve ser correto");
    }

    #[tokio::test]
    async fn test_find_possible_trades_empty_result() {
        // Arrange
        let mock_repository = Arc::new(MockTradeRepository::new(vec![]));
        let trade_service = TradeServiceImpl::new(mock_repository);
        let user_id = Uuid::new_v4();

        // Act
        let result = trade_service.find_possible_trades(user_id).await;

        // Assert
        assert!(result.is_ok(), "Deve executar sem erros mesmo sem trocas");
        let trades = result.unwrap();
        assert_eq!(trades.len(), 0, "Deve retornar lista vazia quando não há trocas");
    }

    #[tokio::test]
    async fn test_find_possible_trades_multiple_results() {
        // Arrange
        let mock_trade1 = create_mock_trade();
        let mut mock_trade2 = create_mock_trade();
        mock_trade2.offered_book.title = "Different Book".to_string();
        mock_trade2.trade_partner.name = "Different Partner".to_string();

        let mock_repository = Arc::new(MockTradeRepository::new(vec![mock_trade1, mock_trade2]));
        let trade_service = TradeServiceImpl::new(mock_repository);
        let user_id = Uuid::new_v4();

        // Act
        let result = trade_service.find_possible_trades(user_id).await;

        // Assert
        assert!(result.is_ok(), "Deve retornar múltiplas trocas com sucesso");
        let trades = result.unwrap();
        assert_eq!(trades.len(), 2, "Deve retornar duas trocas");
        
        let titles: Vec<&str> = trades.iter().map(|t| t.offered_book.title.as_str()).collect();
        assert!(titles.contains(&"Test Book 1"), "Deve incluir primeiro livro");
        assert!(titles.contains(&"Different Book"), "Deve incluir segundo livro");
    }

    #[tokio::test]
    async fn test_find_possible_trades_repository_error() {
        // Arrange
        let mock_repository = Arc::new(MockTradeRepository::new_with_error());
        let trade_service = TradeServiceImpl::new(mock_repository);
        let user_id = Uuid::new_v4();

        // Act
        let result = trade_service.find_possible_trades(user_id).await;

        // Assert
        assert!(result.is_err(), "Deve propagar erro do repositório");
        match result.unwrap_err() {
            AppError::DatabaseError(msg) => {
                assert!(msg.contains("Database connection failed"), "Deve conter mensagem de erro específica");
            }
            _ => panic!("Tipo de erro inesperado"),
        }
    }

    #[tokio::test]
    async fn test_trade_service_creation() {
        // Arrange
        let mock_repository = Arc::new(MockTradeRepository::new(vec![]));

        // Act
        let _trade_service = TradeServiceImpl::new(mock_repository);

        // Assert
        // A verificação é implícita - se compilou e executou, o serviço foi criado corretamente
        assert!(true, "TradeService foi criado com sucesso");
    }
} 