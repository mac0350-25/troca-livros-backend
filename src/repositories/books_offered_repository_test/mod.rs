pub mod create_books_offered_test;
pub mod find_books_offered_test;
pub mod delete_books_offered_test;
pub mod find_by_user_id_test;

use crate::models::book::GoogleBookDto;
use crate::models::user::CreateUserDto;
use crate::repositories::book_repository::{BookRepository, PgBookRepository};
use crate::repositories::books_offered_repository::{BooksOfferedRepository, PgBooksOfferedRepository};
use crate::repositories::test_helpers::{get_test_db_pool, clean_database};
use crate::repositories::user_repository::{UserRepository, PgUserRepository};
use uuid::Uuid;

// Cria uma função de setup de repositório para o PgBookRepository
pub async fn setup_book_repository() -> impl BookRepository {
    let pool = get_test_db_pool().await;
    clean_database(&pool).await;
    PgBookRepository::new(pool)
}

// Cria uma função de setup de repositório para o PgUserRepository
pub async fn setup_user_repository() -> impl UserRepository {
    let pool = get_test_db_pool().await;
    clean_database(&pool).await;
    PgUserRepository::new(pool)
}

// Função para criar o repositório de books_offered
pub async fn setup_test_repository() -> impl BooksOfferedRepository {
    let pool = get_test_db_pool().await;
    clean_database(&pool).await;
    PgBooksOfferedRepository::new(pool)
}

// Função para criar um usuário de teste
pub fn create_test_user() -> CreateUserDto {
    CreateUserDto {
        name: "Test User".to_string(),
        email: format!("test_{}@example.com", Uuid::new_v4()),
        password: "password".to_string(),
    }
}

// Função para criar um livro de teste
pub fn create_test_book(google_id: &str) -> GoogleBookDto {
    GoogleBookDto {
        google_id: google_id.to_string(),
        title: "Livro de Teste".to_string(),
        authors: Some("Autor Teste".to_string()),
        publisher: Some("Editora Teste".to_string()),
        published_date: Some("2022-05-10".to_string()),
        description: Some("Descrição de teste".to_string()),
        image_url: Some("http://example.com/image.jpg".to_string()),
        page_count: Some(200),
    }
} 