mod common;

use crate::common::test_utils::{get_auth_token, get_test_mutex, setup_test_app};
use reqwest::{header, StatusCode};
use serde_json::{json, Value};

#[tokio::test]
async fn test_get_user_books() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;
    
    // Arrange - Configurar o aplicativo de teste e autenticar
    let app = setup_test_app().await;
    let token = get_auth_token(&app).await;
    let client = reqwest::Client::new();

    // Primeiro, vamos buscar um livro para adicionar à lista
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

    // Adicionar um livro à lista de possuídos
    let offered_response = client
        .post(&format!("http://localhost:{}/api/books/offered", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "google_id": google_id
        }))
        .send()
        .await
        .expect("Falha ao adicionar livro à lista de possuídos");

    assert_eq!(offered_response.status(), StatusCode::CREATED);

    // Adicionar um livro diferente à lista de desejados
    // Buscar outro livro para não causar conflito
    let second_search_response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "query": "Domain-Driven Design"
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição da segunda busca");

    let second_search_body: Value = second_search_response
        .json()
        .await
        .expect("Falha ao ler corpo da segunda resposta de busca");

    let second_books = second_search_body["data"].as_array().unwrap();
    assert!(!second_books.is_empty(), "A segunda busca deve retornar pelo menos um livro");

    let second_google_id = second_books[0]["google_id"].as_str().unwrap();

    // Adicionar à lista de desejados
    let wanted_response = client
        .post(&format!("http://localhost:{}/api/books/wanted", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "google_id": second_google_id
        }))
        .send()
        .await
        .expect("Falha ao adicionar livro à lista de desejados");

    assert_eq!(wanted_response.status(), StatusCode::CREATED);

    // Act - Obter livros do usuário
    let response = client
        .get(&format!("http://localhost:{}/api/books", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .expect("Falha ao obter livros do usuário");

    // Assert - Verificar o resultado
    let status = response.status();
    let body: Value = response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta");

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "success");
    assert_eq!(
        body["message"],
        "Livros do usuário recuperados com sucesso"
    );
    
    // Verificar se as listas estão presentes
    assert!(body["data"]["offered_books"].is_array());
    assert!(body["data"]["wanted_books"].is_array());
    
    // Verificar se tem pelo menos um livro em cada lista
    assert!(body["data"]["offered_books"].as_array().unwrap().len() >= 1);
    assert!(body["data"]["wanted_books"].as_array().unwrap().len() >= 1);
    
    // Limpar depois do teste - remover os livros adicionados
    
    // Obter IDs dos livros adicionados
    let offered_book_id = body["data"]["offered_books"][0]["id"].as_str().unwrap();
    let wanted_book_id = body["data"]["wanted_books"][0]["id"].as_str().unwrap();
    
    // Remover livro possuído
    let offered_delete_response = client
        .delete(&format!("http://localhost:{}/api/books/offered/{}", app.port, offered_book_id))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .expect("Falha ao remover livro da lista de possuídos");
    
    assert_eq!(offered_delete_response.status(), StatusCode::OK);
    
    // Remover livro desejado
    let wanted_delete_response = client
        .delete(&format!("http://localhost:{}/api/books/wanted/{}", app.port, wanted_book_id))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .expect("Falha ao remover livro da lista de desejados");
    
    assert_eq!(wanted_delete_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_user_books_without_authentication() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;
    
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Act - Tentar obter livros sem autenticação
    let response = client
        .get(&format!("http://localhost:{}/api/books", app.port))
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