use serde_json::json;
use crate::error::AppError;
use crate::services::http_service::{HttpService, HttpServiceImpl};

// Implementação simples de HttpService para testes
struct TestHttpService {
    response: Option<serde_json::Value>,
    error: Option<AppError>,
    expected_url: String,
}

impl TestHttpService {
    fn new_success(url: &str, response: serde_json::Value) -> Self {
        Self {
            response: Some(response),
            error: None,
            expected_url: url.to_string(),
        }
    }

    fn new_error(url: &str, error: AppError) -> Self {
        Self {
            response: None,
            error: Some(error),
            expected_url: url.to_string(),
        }
    }
}

impl HttpService for TestHttpService {
    fn get<'a>(
        &'a self,
        url: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<serde_json::Value, AppError>> + Send + 'a>> {
        let response = self.response.clone();
        let error = self.error.clone();
        let expected_url = self.expected_url.clone();

        Box::pin(async move {
            // Verifica se a URL é a esperada
            assert_eq!(url, expected_url, "URL não corresponde à esperada");

            // Retorna o resultado configurado
            match (response, error) {
                (Some(resp), _) => Ok(resp),
                (_, Some(err)) => Err(err),
                _ => Err(AppError::InternalServerError("Nenhuma resposta ou erro configurado".to_string())),
            }
        })
    }
}

// Teste para simular erro na requisição HTTP
#[tokio::test(flavor = "current_thread")]
async fn test_http_request_error() {
    // Criar serviço de teste que simula um erro de conexão
    let service = TestHttpService::new_error(
        "https://example.com/test",
        AppError::InternalServerError("Erro na requisição HTTP: falha de conexão".to_string())
    );
    
    // Realizar a requisição
    let result = service.get("https://example.com/test").await;
    
    // Verificar que retornou o erro interno esperado
    assert!(result.is_err());
    match result {
        Err(AppError::InternalServerError(msg)) => {
            assert!(msg.contains("Erro na requisição HTTP"));
        }
        _ => panic!("Deveria ser um erro InternalServerError"),
    }
}

// Teste para simular erro de status HTTP (404)
#[tokio::test(flavor = "current_thread")]
async fn test_http_status_error() {
    // Criar serviço de teste que simula um erro 404
    let service = TestHttpService::new_error(
        "https://example.com/not-found",
        AppError::NotFoundError("Erro na requisição: Status 404".to_string())
    );
    
    // Realizar a requisição
    let result = service.get("https://example.com/not-found").await;
    
    // Verificar que retornou o erro NotFound esperado
    assert!(result.is_err());
    match result {
        Err(AppError::NotFoundError(msg)) => {
            assert!(msg.contains("Status 404"));
        }
        _ => panic!("Deveria ser um erro NotFoundError"),
    }
}

// Teste para simular JSON inválido
#[tokio::test(flavor = "current_thread")]
async fn test_invalid_json_response() {
    // Criar serviço de teste que simula um erro de JSON inválido
    let service = TestHttpService::new_error(
        "https://example.com/invalid-json",
        AppError::InternalServerError("Erro ao processar resposta: JSON inválido".to_string())
    );
    
    // Realizar a requisição
    let result = service.get("https://example.com/invalid-json").await;
    
    // Verificar que retornou o erro de processamento esperado
    assert!(result.is_err());
    match result {
        Err(AppError::InternalServerError(msg)) => {
            assert!(msg.contains("Erro ao processar resposta"));
        }
        _ => panic!("Deveria ser um erro InternalServerError"),
    }
}

// Teste para resposta bem-sucedida
#[tokio::test(flavor = "current_thread")]
async fn test_successful_response() {
    // Criar serviço de teste que simula uma resposta bem-sucedida
    let service = TestHttpService::new_success(
        "https://example.com/success",
        json!({"test": "success"})
    );
    
    // Realizar a requisição
    let result = service.get("https://example.com/success").await;
    
    // Verificar que retornou o resultado esperado
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data["test"], json!("success"));
}

// Teste de integração com o HttpServiceImpl real
#[tokio::test(flavor = "current_thread")]
async fn test_real_http_service() {
    // Criamos uma instância real do HttpService
    let http_service = HttpServiceImpl::new();
    
    // Fazemos uma requisição para uma API pública conhecida
    let result = http_service.get("https://jsonplaceholder.typicode.com/todos/1").await;
    
    // Verificamos se a requisição foi bem-sucedida
    assert!(result.is_ok());
    
    // Verificamos se a resposta contém campos esperados
    let data = result.unwrap();
    assert!(data.get("id").is_some());
    assert!(data.get("title").is_some());
}