mod common;

use crate::common::test_utils::{get_auth_token, get_test_mutex, setup_test_app};
use reqwest::{header, StatusCode};
use serde_json::{json, Value};

#[tokio::test]
async fn test_add_book_to_wanted() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;
    
    // Arrange - Configurar o aplicativo de teste e autenticar
    let app = setup_test_app().await;
    let token = get_auth_token(&app).await;
    let client = reqwest::Client::new();

    // Primeiro, vamos buscar um livro para obter um ID válido
    let search_response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "query": "Clean Code"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição de busca");

    let search_body: Value = search_response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta de busca");

    let books = search_body["data"].as_array().unwrap();
    assert!(!books.is_empty(), "A busca deve retornar pelo menos um livro");

    let google_id = books[0]["google_id"].as_str().unwrap();

    // Act - Adicionar o livro à lista de desejados
    let response = client
        .post(&format!("http://localhost:{}/api/books/wanted", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "google_id": google_id
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição para adicionar livro");

    // Assert - Verificar o resultado
    let status = response.status();
    let body: Value = response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta");

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["status"], "success");
    assert_eq!(
        body["message"],
        "Livro adicionado à lista de desejados com sucesso"
    );
    assert!(body["data"]["book_id"].is_string());
    assert!(body["data"]["user_id"].is_string());

    // Tentar adicionar o mesmo livro novamente deve falhar
    let duplicate_response = client
        .post(&format!("http://localhost:{}/api/books/wanted", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "google_id": google_id
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição duplicada");

    let duplicate_status = duplicate_response.status();
    let duplicate_body: Value = duplicate_response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta duplicada");

    assert_eq!(duplicate_status, StatusCode::BAD_REQUEST);
    assert_eq!(duplicate_body["error"]["status"], 400);
    assert!(duplicate_body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("já está na sua lista de desejados"));
}

#[tokio::test]
async fn test_add_book_to_wanted_invalid_google_id() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;
    
    // Arrange - Configurar o aplicativo de teste e autenticar
    let app = setup_test_app().await;
    let token = get_auth_token(&app).await;
    let client = reqwest::Client::new();

    // Act - Tentar adicionar um livro com ID inválido
    let response = client
        .post(&format!("http://localhost:{}/api/books/wanted", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "google_id": "id_que_nao_existe_12345"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição");

    // Assert - A requisição deve falhar pois o Google Books API não encontrará o livro
    let status = response.status();
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_add_book_to_wanted_without_authentication() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;
    
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Act - Tentar adicionar um livro sem autenticação
    let response = client
        .post(&format!("http://localhost:{}/api/books/wanted", app.port))
        .json(&json!({
            "google_id": "qualquerid"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição");

    // Assert - A requisição deve falhar por falta de autenticação
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
        .contains("Token de autenticação ausente"));
}

// Teste para verificar erro ao tentar adicionar livro que já está na lista de possuídos
#[tokio::test]
async fn test_add_book_to_wanted_already_in_offered() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;
    
    // Arrange - Configurar o aplicativo de teste e autenticar
    let app = setup_test_app().await;
    let token = get_auth_token(&app).await;
    let client = reqwest::Client::new();

    // Primeiro, vamos buscar um livro para obter um ID válido
    let search_response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "query": "Clean Code"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição de busca");

    let search_body: Value = search_response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta de busca");

    let books = search_body["data"].as_array().unwrap();
    assert!(!books.is_empty(), "A busca deve retornar pelo menos um livro");

    let google_id = books[0]["google_id"].as_str().unwrap();

    // Adicionar à lista de possuídos 
    let offered_response = client
        .post(&format!("http://localhost:{}/api/books/offered", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "google_id": google_id
        }))
        .send()
        .await;

    // Garantir que o livro foi adicionado à lista de possuídos
    assert_eq!(offered_response.unwrap().status(), StatusCode::CREATED);

    // Act - Tentar adicionar o mesmo livro à lista de desejados
    let response = client
        .post(&format!("http://localhost:{}/api/books/wanted", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "google_id": google_id
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição para adicionar livro à lista de desejados");

    // Assert - Verificar que a requisição falhou com o erro esperado
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
        .contains("já está na sua lista de possuídos"));
}
