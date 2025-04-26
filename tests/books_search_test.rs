mod common;

use crate::common::test_utils::setup_test_app;
use reqwest::StatusCode;
use serde_json::{json, Value};

#[tokio::test]
async fn test_search_books_success() {
    // Arrange - Configurar o aplicativo de teste
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Act - Buscar livros com um termo de busca válido
    let response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .json(&json!({
            "query": "Rust Programming"
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

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "success");
    assert_eq!(body["message"], "Livros encontrados com sucesso");
    assert!(body["data"].is_array());

    // Verificar se há pelo menos um livro na resposta
    let data = body["data"].as_array().unwrap();
    if !data.is_empty() {
        let first_book = &data[0];

        // Verificar se os campos obrigatórios existem
        assert!(first_book["google_id"].is_string());
        assert!(first_book["title"].is_string());

        // Verificar se os campos têm valores adequados
        assert!(!first_book["google_id"].as_str().unwrap().is_empty());
        assert!(!first_book["title"].as_str().unwrap().is_empty());
    }
}

#[tokio::test]
async fn test_search_books_empty_query() {
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Act - Enviar uma consulta vazia
    let response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .json(&json!({
            "query": ""
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
        .contains("não pode estar vazia"));
}

#[tokio::test]
async fn test_search_books_specific_book() {
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Act - Buscar um livro específico com termos mais específicos
    let response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .json(&json!({
            "query": "Clean Code: A Handbook of Agile Software Craftsmanship Robert Martin"
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

    assert_eq!(status, StatusCode::OK);

    let data = body["data"].as_array().unwrap();

    // Verificar se os resultados são relevantes - não exigimos match exato, pois a API pode retornar resultados diferentes
    let is_relevant = data.iter().any(|book| {
        let title = book["title"].as_str().unwrap_or("");
        let authors = book["authors"].as_str().unwrap_or("");

        // Considerar relevante se contiver partes do título ou nome do autor
        title.contains("Clean")
            || title.contains("Code")
            || authors.contains("Martin")
            || authors.contains("Robert")
    });

    assert!(
        is_relevant,
        "Deveria encontrar livros relacionados a 'Clean Code' ou 'Robert Martin'"
    );
}

#[tokio::test]
async fn test_search_books_nonexistent_title() {
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Act - Buscar um título improvável de existir
    let unique_query = format!("TítuloMuitoImprovável{}", chrono::Utc::now().timestamp());
    let response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .json(&json!({
            "query": unique_query
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

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "success");

    // O array de dados deve estar vazio ou ter poucos resultados
    let data = body["data"].as_array().unwrap();
    assert!(
        data.len() < 2,
        "Não deveria encontrar muitos livros com um título improvável"
    );
}

#[tokio::test]
async fn test_search_books_with_author_filter() {
    // Arrange
    let app = setup_test_app().await;
    let client = reqwest::Client::new();

    // Act - Buscar livros de um autor específico
    let response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .json(&json!({
            "query": "author:Martin Fowler"
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

    assert_eq!(status, StatusCode::OK);

    let data = body["data"].as_array().unwrap();
    if !data.is_empty() {
        // Verificar se os livros retornados são do autor pesquisado
        let author_found = data.iter().any(|book| {
            if let Some(authors) = book["authors"].as_str() {
                authors.contains("Fowler") || authors.contains("Martin")
            } else {
                false
            }
        });

        assert!(
            author_found,
            "Deveria encontrar pelo menos um livro do autor Martin Fowler"
        );
    }
}
