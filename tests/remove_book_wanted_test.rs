mod common;

use crate::common::test_utils::{get_auth_token, setup_test_app};
use reqwest::{header, StatusCode};
use serde_json::{json, Value};


#[tokio::test]
async fn test_remove_book_from_wanted() {
    // Arrange - Configurar o aplicativo de teste e autenticar
    let app = setup_test_app().await;
    let token = get_auth_token(&app).await;
    let client = reqwest::Client::new();

    // Primeiro, vamos buscar um livro para obter um ID válido
    let search_response = client
        .post(&format!("http://localhost:{}/api/books/search", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "query": "Domain-Driven Design"
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

    // Adicionar o livro à lista de desejados
    let add_response = client
        .post(&format!("http://localhost:{}/api/books/wanted", app.port))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&json!({
            "google_id": google_id
        }))
        .send()
        .await
        .expect("Falha ao enviar requisição para adicionar livro");

    let add_body: Value = add_response
        .json()
        .await
        .expect("Falha ao ler corpo da resposta de adição");

    let book_id = add_body["data"]["book_id"].as_str().unwrap();

    // Act - Remover o livro da lista de desejados
    let response = client
        .delete(&format!(
            "http://localhost:{}/api/books/wanted/{}",
            app.port, book_id
        ))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .expect("Falha ao enviar requisição para remover livro");

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
        "Livro removido da lista de desejados com sucesso"
    );

    // Tentar remover o mesmo livro novamente deve falhar
    let second_delete_response = client
        .delete(&format!(
            "http://localhost:{}/api/books/wanted/{}",
            app.port, book_id
        ))
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .expect("Falha ao enviar segunda requisição para remover livro");

    let second_delete_status = second_delete_response.status();
    let second_delete_body: Value = second_delete_response
        .json()
        .await
        .expect("Falha ao ler corpo da segunda resposta de remoção");

    assert_eq!(second_delete_status, StatusCode::BAD_REQUEST);
    assert_eq!(second_delete_body["error"]["status"], 400);
    assert!(second_delete_body["error"]["message"]
        .as_str()
        .unwrap()
        .contains("não está na sua lista de desejados"));
} 