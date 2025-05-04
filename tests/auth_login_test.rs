mod common;

use crate::common::test_utils::{get_test_mutex, setup_test_app};
use reqwest::StatusCode;
use serde_json::{json, Value};

#[tokio::test]
async fn test_login_success() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;
    
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Criar email único para evitar conflitos
    let email = format!("login_test_{}@example.com", chrono::Utc::now().timestamp());
    let password = "senha123";

    // Registrar um usuário primeiro
    let register_response = client
        .post(&format!("http://localhost:{}/api/auth/register", app.port))
        .json(&json!({
            "name": "Usuário de Login",
            "email": email,
            "password": password
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição de registro");

    assert_eq!(register_response.status(), StatusCode::CREATED);

    // Act - Fazer login com usuário criado
    let login_response = client
        .post(&format!("http://localhost:{}/api/auth/login", app.port))
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição de login");

    // Assert
    let status = login_response.status();
    let body: Value = login_response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta");

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "success");
    assert_eq!(body["message"], "Login realizado com sucesso");

    // Verificar se retornou um token e os dados do usuário
    assert!(body["data"]["access_token"].is_string());
    assert_eq!(body["data"]["token_type"], "Bearer");
    assert_eq!(body["data"]["user"]["email"], email);
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;
    
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Criar email único para evitar conflitos
    let email = format!(
        "invalid_login_{}@example.com",
        chrono::Utc::now().timestamp()
    );

    // Registrar um usuário primeiro
    let register_response = client
        .post(&format!("http://localhost:{}/api/auth/register", app.port))
        .json(&json!({
            "name": "Usuário para Teste de Login Inválido",
            "email": email,
            "password": "senha123"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição de registro");

    assert_eq!(register_response.status(), StatusCode::CREATED);

    // Act - Fazer login com senha incorreta
    let login_response = client
        .post(&format!("http://localhost:{}/api/auth/login", app.port))
        .json(&json!({
            "email": email,
            "password": "senha_incorreta"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição de login");

    // Assert
    let status = login_response.status();
    let body: Value = login_response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta");

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["status"], 401);
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Credenciais inválidas"));
}

#[tokio::test]
async fn test_login_nonexistent_user() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;
    
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Email que provavelmente não existe
    let email = format!("nonexistent_{}@example.com", chrono::Utc::now().timestamp());

    // Act - Fazer login com usuário inexistente
    let login_response = client
        .post(&format!("http://localhost:{}/api/auth/login", app.port))
        .json(&json!({
            "email": email,
            "password": "qualquer_senha"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição de login");

    // Assert
    let status = login_response.status();
    let body: Value = login_response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta");

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["status"], 401);
    assert!(body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Credenciais inválidas"));
}
