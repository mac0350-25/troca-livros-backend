use std::sync::Arc;

use mockall::predicate::*;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::book::{BookOffered, CreateBookOfferedDto, GoogleBookDto};
use crate::repositories::book_repository::BookWithId;
use crate::services::book_offered_service::{BookOfferedService, BookOfferedServiceImpl};
use crate::services::book_offered_service_test::{MockBookRepository, MockBooksOfferedRepository, MockGoogleBookService};

#[tokio::test]
async fn test_add_book_to_offered_when_book_exists() {
    // Arrange
    let mut book_repo = MockBookRepository::new();
    let mut books_offered_repo = MockBooksOfferedRepository::new();
    
    let google_id = "test123";
    let user_id = Uuid::new_v4();
    let book_id = Uuid::new_v4();

    // Configurar o mock do book_repository para retornar Some para find_by_google_id
    book_repo
        .expect_find_by_google_id()
        .with(eq(google_id))
        .times(1)
        .returning(move |_| {
            Ok(Some(BookWithId {
                id: book_id,
                book: GoogleBookDto {
                    google_id: google_id.to_string(),
                    title: "Livro Teste".to_string(),
                    authors: None,
                    publisher: None,
                    published_date: None,
                    description: None,
                    image_url: None,
                    page_count: None,
                }
            }))
        });

    // Configurar o mock do books_offered_repository para retornar None para find
    // (indicando que o livro ainda não está na lista de possuídos)
    books_offered_repo
        .expect_find()
        .with(eq(book_id), eq(user_id))
        .times(1)
        .returning(|_, _| Ok(None));

    // Configurar o mock do books_offered_repository para retornar um BookOffered para create
    books_offered_repo
        .expect_create()
        .with(function(move |dto: &CreateBookOfferedDto| {
            dto.book_id == book_id && dto.user_id == user_id
        }))
        .times(1)
        .returning(move |dto| {
            Ok(BookOffered {
                book_id: dto.book_id,
                user_id: dto.user_id,
            })
        });

    // Google Book Service não será usado neste teste, pois o livro já existe
    let google_book_service = MockGoogleBookService::new();

    // Act
    let service = BookOfferedServiceImpl::new(
        Arc::new(book_repo),
        Arc::new(books_offered_repo),
        Arc::new(google_book_service),
    );

    let result = service.add_book_to_offered(google_id, &user_id).await;

    // Assert
    assert!(result.is_ok());
    let book_offered = result.unwrap();
    assert_eq!(book_offered.book_id, book_id);
    assert_eq!(book_offered.user_id, user_id);
}

#[tokio::test]
async fn test_add_book_to_offered_when_book_does_not_exist() {
    // Arrange
    let mut book_repo = MockBookRepository::new();
    let mut books_offered_repo = MockBooksOfferedRepository::new();

    let google_id = "new_book123";
    let user_id = Uuid::new_v4();
    let book_id = Uuid::new_v4();

    // Configurar o mock do book_repository para retornar None para find_by_google_id
    book_repo
        .expect_find_by_google_id()
        .with(eq(google_id))
        .times(1)
        .returning(|_| Ok(None));

    // Configurar o mock do google_book_service para retornar um livro
    let google_book_service = MockGoogleBookService::new()
        .with_find_book_by_id(move |id| {
            assert_eq!(id, google_id);
            Ok(GoogleBookDto {
                google_id: google_id.to_string(),
                title: "Novo Livro".to_string(),
                authors: None,
                publisher: None,
                published_date: None,
                description: None,
                image_url: None,
                page_count: None,
            })
        });

    // Configurar o mock do book_repository para retornar um UUID para create
    book_repo
        .expect_create()
        .times(1)
        .returning(move |_| Ok(book_id));

    // Configurar o mock do books_offered_repository para retornar None para find
    // (indicando que o livro ainda não está na lista de possuídos)
    books_offered_repo
        .expect_find()
        .with(eq(book_id), eq(user_id))
        .times(1)
        .returning(|_, _| Ok(None));

    // Configurar o mock do books_offered_repository para retornar um BookOffered para create
    books_offered_repo
        .expect_create()
        .with(function(move |dto: &CreateBookOfferedDto| {
            dto.book_id == book_id && dto.user_id == user_id
        }))
        .times(1)
        .returning(move |dto| {
            Ok(BookOffered {
                book_id: dto.book_id,
                user_id: dto.user_id,
            })
        });

    // Act
    let service = BookOfferedServiceImpl::new(
        Arc::new(book_repo),
        Arc::new(books_offered_repo),
        Arc::new(google_book_service),
    );

    let result = service.add_book_to_offered(google_id, &user_id).await;

    // Assert
    assert!(result.is_ok());
    let book_offered = result.unwrap();
    assert_eq!(book_offered.book_id, book_id);
    assert_eq!(book_offered.user_id, user_id);
}

#[tokio::test]
async fn test_add_book_to_offered_when_book_already_offered() {
    // Arrange
    let mut book_repo = MockBookRepository::new();
    let mut books_offered_repo = MockBooksOfferedRepository::new();

    let google_id = "already_offered123";
    let user_id = Uuid::new_v4();
    let book_id = Uuid::new_v4();

    // Configurar o mock do book_repository para retornar Some para find_by_google_id
    book_repo
        .expect_find_by_google_id()
        .with(eq(google_id))
        .times(1)
        .returning(move |_| {
            Ok(Some(BookWithId {
                id: book_id,
                book: GoogleBookDto {
                    google_id: google_id.to_string(),
                    title: "Livro Já Oferecido".to_string(),
                    authors: None,
                    publisher: None,
                    published_date: None,
                    description: None,
                    image_url: None,
                    page_count: None,
                }
            }))
        });

    // Configurar o mock do books_offered_repository para retornar Some para find
    // (indicando que o livro já está na lista de possuídos)
    books_offered_repo
        .expect_find()
        .with(eq(book_id), eq(user_id))
        .times(1)
        .returning(move |book_id, user_id| {
            Ok(Some(BookOffered {
                book_id: *book_id,
                user_id: *user_id,
            }))
        });

    // Google Book Service não será usado neste teste
    let google_book_service = MockGoogleBookService::new();

    // Act
    let service = BookOfferedServiceImpl::new(
        Arc::new(book_repo),
        Arc::new(books_offered_repo),
        Arc::new(google_book_service),
    );

    let result = service.add_book_to_offered(google_id, &user_id).await;

    // Assert
    assert!(result.is_err());
    match result {
        Err(AppError::ValidationError(msg)) => {
            assert_eq!(msg, "Este livro já está na sua lista de possuídos");
        }
        _ => panic!("Erro inesperado"),
    }
} 