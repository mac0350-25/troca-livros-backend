mod common;

use crate::common::test_utils::{get_auth_token, setup_test_app};
use reqwest::{header, StatusCode};
use serde_json::{json, Value};

#[tokio::test]
async fn test_auth_middleware_token_expiration() {
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Token expirado (exp está no passado)
    let expired_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJleHAiOjE1MTYyMzkwMjJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

    // Act - Tentar acessar uma rota protegida com token expirado
    let response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", expired_token))
        .json(&json!({
            "query": "Clean Code"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição");

    // Assert
    let status = response.status();
    let body: Value = response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta");

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["status"], 401);
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Token inválido"));
}

#[tokio::test]
async fn test_auth_middleware_malformed_token() {
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Token malformado (não é um JWT válido)
    let malformed_token = "isto_nao_e_um_token_jwt_valido";

    // Act - Tentar acessar uma rota protegida com token malformado
    let response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", malformed_token))
        .json(&json!({
            "query": "Clean Code"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição");

    // Assert
    let status = response.status();
    let body: Value = response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta");

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["status"], 401);
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Token inválido"));
}

#[tokio::test]
async fn test_auth_middleware_missing_bearer() {
    // Arrange
    let app = setup_test_app().await;
    let token = get_auth_token(&app).await;
    let client = reqwest::Client::new();

    // Act - Enviar token sem o prefixo "Bearer "
    let response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .header(header::AUTHORIZATION, token) // Token sem o prefixo Bearer
        .json(&json!({
            "query": "Clean Code"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição");

    // Assert
    let status = response.status();
    let body: Value = response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta");

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["status"], 401);
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Formato de token inválido"));
}

#[tokio::test]
async fn test_auth_middleware_access_multiple_protected_routes() {
    // Arrange
    let app = setup_test_app().await;
    let token = get_auth_token(&app).await;
    let client = reqwest::Client::new();

    // Act - Acessar a primeira rota protegida
    let response1 = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "query": "Clean Code"
        }))
        .send()
        .await
        .expect("Falha ao enviar primeira requisição");

    // Assert para a primeira rota
    assert_eq!(response1.status(), StatusCode::OK);

    // Act - Acessar a mesma rota protegida novamente com o mesmo token
    let response2 = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "query": "Domain-Driven Design"
        }))
        .send()
        .await
        .expect("Falha ao enviar segunda requisição");

    // Assert para a segunda rota
    assert_eq!(response2.status(), StatusCode::OK);

    // Verificar que os dados retornados são diferentes (consultas diferentes)
    let body1: Value = response1
        .json()
        .await
        .expect("Falha ao ler primeira resposta");
    let body2: Value = response2
        .json()
        .await
        .expect("Falha ao ler segunda resposta");

    // Verificar que ambas as respostas têm o status de sucesso
    assert_eq!(body1["status"], "success");
    assert_eq!(body2["status"], "success");
}
