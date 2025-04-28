use crate::services::google_book_service::{GoogleBookService, GoogleBookServiceImpl};
use crate::services::http_service::{HttpService, HttpServiceImpl};
use std::sync::Arc;
use crate::error::AppError;
use serde_json::json;

// Implementação para mock do HttpService
struct MockHttpService {
    response: Option<serde_json::Value>,
    error: Option<AppError>,
}

impl MockHttpService {
    fn new_success(response: serde_json::Value) -> Self {
        Self {
            response: Some(response),
            error: None,
        }
    }

    fn new_error(error: AppError) -> Self {
        Self {
            response: None,
            error: Some(error),
        }
    }
}

impl HttpService for MockHttpService {
    fn get<'a>(
        &'a self,
        _url: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<serde_json::Value, AppError>> + Send + 'a>> {
        let response = self.response.clone();
        let error = self.error.clone();

        Box::pin(async move {
            match (response, error) {
                (Some(resp), _) => Ok(resp),
                (_, Some(err)) => Err(err),
                _ => Err(AppError::InternalServerError("Nenhuma resposta ou erro configurado".to_string())),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn find_book_by_id_test() {
        // Arrange
        let http_service = Arc::new(HttpServiceImpl::new());
        let service = GoogleBookServiceImpl::new(http_service);
        // ID de um livro bem conhecido
        let book_id = "YyXoAAAACAAJ"; // Clean Code de Robert Martin

        // Act
        let result = service.find_book_by_id(book_id).await;

        // Assert
        assert!(result.is_ok(), "A busca de livro por ID deveria ser bem-sucedida");

        let book = result.unwrap();
        assert_eq!(book.google_id, book_id, "O ID do livro deve corresponder ao solicitado");
        assert!(!book.title.is_empty(), "O título não deve estar vazio");
    }

    #[tokio::test]
    async fn test_find_book_by_id_invalid_id() {
        // Arrange
        let http_service = Arc::new(HttpServiceImpl::new());
        let service = GoogleBookServiceImpl::new(http_service);
        // ID inválido que não deve existir
        let book_id = "invalid_id_12345678900987654321";

        // Act
        let result = service.find_book_by_id(book_id).await;

        // Assert
        assert!(result.is_err(), "A busca com ID inválido deve falhar");
    }

    #[tokio::test]
    async fn test_find_book_by_id_empty_id() {
        // Arrange
        let http_service = Arc::new(HttpServiceImpl::new());
        let service = GoogleBookServiceImpl::new(http_service);
        let book_id = "";

        // Act
        let result = service.find_book_by_id(book_id).await;

        // Assert
        assert!(result.is_err(), "A busca com ID vazio deve falhar");
    }

    #[tokio::test]
    async fn test_find_book_by_id_real_book_details() {
        // Arrange
        let http_service = Arc::new(HttpServiceImpl::new());
        let service = GoogleBookServiceImpl::new(http_service);
        // ID de um livro específico da O'Reilly sobre Rust
        let book_id = "0weDoAEACAAJ"; // "Programming Rust" de Jim Blandy

        // Act
        let result = service.find_book_by_id(book_id).await;

        // Assert
        assert!(result.is_ok(), "A busca por ID válido deve ser bem-sucedida");

        let book = result.unwrap();
        assert_eq!(book.google_id, book_id);
        
        // Validações de campos obrigatórios
        assert!(!book.title.is_empty(), "Título deve estar presente");
        
        // Verificação de campos opcionais com conteúdo esperado
        if let Some(authors) = book.authors {
            assert!(!authors.is_empty(), "Lista de autores não deve estar vazia quando presente");
        }
        
        if let Some(description) = book.description {
            assert!(!description.is_empty(), "Descrição não deve estar vazia quando presente");
        }
        
        if let Some(image_url) = book.image_url {
            assert!(image_url.starts_with("http"), "URL da imagem deve ser válida");
        }
    }

    // Teste usando mock para simular erro 404 (NotFoundError)
    #[tokio::test]
    async fn test_find_book_by_id_not_found_error() {
        // Arrange
        // Criar um mock do HttpService que retorna erro NotFoundError
        let mock_http_service = Arc::new(MockHttpService::new_error(
            AppError::NotFoundError("Recurso não encontrado".to_string())
        ));
        
        let service = GoogleBookServiceImpl::new(mock_http_service);
        let book_id = "test_not_found_id";

        // Act
        let result = service.find_book_by_id(book_id).await;

        // Assert
        assert!(result.is_err(), "Deve falhar quando o livro não é encontrado");
        
        match result {
            Err(AppError::NotFoundError(msg)) => {
                assert!(msg.contains("não encontrado"), "Mensagem deve informar que o livro não foi encontrado");
                assert!(msg.contains(book_id), "Mensagem deve conter o ID do livro");
            },
            _ => panic!("Deveria retornar um NotFoundError")
        }
    }

    // Teste usando mock para simular erro interno (InternalServerError)
    #[tokio::test]
    async fn test_find_book_by_id_internal_error() {
        // Arrange
        // Criar um mock do HttpService que retorna erro interno
        let mock_http_service = Arc::new(MockHttpService::new_error(
            AppError::InternalServerError("Erro interno na requisição HTTP".to_string())
        ));
        
        let service = GoogleBookServiceImpl::new(mock_http_service);
        let book_id = "test_error_id";

        // Act
        let result = service.find_book_by_id(book_id).await;

        // Assert
        assert!(result.is_err(), "Deve falhar quando ocorre um erro interno");
        
        match result {
            Err(AppError::InternalServerError(msg)) => {
                assert!(msg.contains("Erro interno"), "Mensagem deve informar sobre o erro interno");
            },
            _ => panic!("Deveria propagar o InternalServerError")
        }
    }

    // Teste usando mock para simular resposta vazia ou inválida
    #[tokio::test]
    async fn test_find_book_by_id_empty_response() {
        // Arrange
        // Criar um mock que retorna objeto vazio com apenas ID
        let mock_http_service = Arc::new(MockHttpService::new_success(json!({
            "id": "test_empty_response_id",
            // Sem volumeInfo ou outros campos necessários
        })));
        
        let service = GoogleBookServiceImpl::new(mock_http_service);
        let book_id = "test_empty_response_id";

        // Act
        let result = service.find_book_by_id(book_id).await;

        // Assert
        // A chamada deve ter sucesso, mas o livro retornado deve ter apenas o ID
        assert!(result.is_ok(), "Deve retornar um objeto mesmo com dados mínimos");
        
        let book = result.unwrap();
        assert_eq!(book.google_id, book_id, "ID do livro deve corresponder");
        assert_eq!(book.title, "", "Título deve estar vazio");
        assert_eq!(book.authors, None, "Autores devem ser None");
        assert_eq!(book.description, None, "Descrição deve ser None");
    }
}
