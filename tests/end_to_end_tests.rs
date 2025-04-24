mod common;

use crate::common::test_utils::setup_test_app;
use reqwest::StatusCode;
use serde_json::{json, Value};

#[tokio::test]
async fn test_register_user_success() {
    // Arrange - Configurar o aplicativo de teste
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Gerar um email único para evitar conflitos
    let email = format!("test_user_{}@example.com", chrono::Utc::now().timestamp());

    // Act - Enviar requisição para registrar um usuário
    let response = client
        .post(&format!("http://localhost:{}/api/auth/register", app.port))
        .json(&json!({
            "name": "Usuário de Teste",
            "email": email,
            "password": "senha123"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição");

    // Assert - Verificar o resultado
    let status = response.status();
    let body: Value = response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta");

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["status"], "success");
    assert_eq!(body["message"], "Usuário registrado com sucesso");
    assert!(body["data"]["id"].is_string());
    assert_eq!(body["data"]["name"], "Usuário de Teste");
    assert_eq!(body["data"]["email"], email);
}

#[tokio::test]
async fn test_register_user_invalid_email() {
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Act - Enviar requisição com email inválido
    let response = client
        .post(&format!("http://localhost:{}/api/auth/register", app.port))
        .json(&json!({
            "name": "Usuário de Teste",
            "email": "email_invalido",
            "password": "senha123"
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

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["status"], 400);
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Formato de email inválido"));
}

#[tokio::test]
async fn test_register_user_password_too_short() {
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Act - Enviar requisição com senha muito curta
    let response = client
        .post(&format!("http://localhost:{}/api/auth/register", app.port))
        .json(&json!({
            "name": "Usuário de Teste",
            "email": "usuario@example.com",
            "password": "12345"  // Menos de 6 caracteres
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

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["status"], 400);
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("A senha deve ter"));
}

#[tokio::test]
async fn test_register_user_duplicate_email() {
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Email que será duplicado
    let email = format!("duplicate_{}@example.com", chrono::Utc::now().timestamp());

    // Primeiro registro (deve ter sucesso)
    let _ = client
        .post(&format!("http://localhost:{}/api/auth/register", app.port))
        .json(&json!({
            "name": "Primeiro Usuário",
            "email": email,
            "password": "senha123"
        }))
        .send()
        .await
        .expect("Falha ao enviar primeira requisição");

    // Act - Tentar registrar com o mesmo email
    let response = client
        .post(&format!("http://localhost:{}/api/auth/register", app.port))
        .json(&json!({
            "name": "Segundo Usuário",
            "email": email,
            "password": "outrasenha123"
        }))
        .send()
        .await
        .expect("Falha ao enviar segunda requisição");

    // Assert
    let status = response.status();
    let body: Value = response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta");

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["status"], 400);
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Email já está em uso"));
}
