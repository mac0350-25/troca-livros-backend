use std::sync::Arc;

use mockall::predicate::*;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::book::BookOffered;
use crate::services::book_offered_service::{BookOfferedService, BookOfferedServiceImpl};
use crate::services::book_offered_wanted_service_test::{MockBookRepository, MockBooksOfferedRepository, MockBooksWantedRepository, MockGoogleBookService};

#[tokio::test]
async fn test_remove_book_from_offered() {
    // Arrange
    let book_repo = MockBookRepository::new();
    let mut books_offered_repo = MockBooksOfferedRepository::new();
    let books_wanted_repo = MockBooksWantedRepository::new();
    let google_book_service = MockGoogleBookService::new();

    let book_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    // Configurar o mock do books_offered_repository para retornar Some para find
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

    // Configurar o mock do books_offered_repository para retornar true para delete
    books_offered_repo
        .expect_delete()
        .with(eq(book_id), eq(user_id))
        .times(1)
        .returning(|_, _| Ok(true));

    // Act
    let service = BookOfferedServiceImpl::new(
        Arc::new(book_repo),
        Arc::new(books_offered_repo),
        Arc::new(books_wanted_repo),
        Arc::new(google_book_service),
    );

    let result = service.remove_book_from_offered(&book_id, &user_id).await;

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[tokio::test]
async fn test_remove_book_from_offered_when_not_offered() {
    // Arrange
    let book_repo = MockBookRepository::new();
    let mut books_offered_repo = MockBooksOfferedRepository::new();
    let books_wanted_repo = MockBooksWantedRepository::new();
    let google_book_service = MockGoogleBookService::new();

    let book_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    // Configurar o mock do books_offered_repository para retornar None para find
    books_offered_repo
        .expect_find()
        .with(eq(book_id), eq(user_id))
        .times(1)
        .returning(|_, _| Ok(None));

    // Act
    let service = BookOfferedServiceImpl::new(
        Arc::new(book_repo),
        Arc::new(books_offered_repo),
        Arc::new(books_wanted_repo),
        Arc::new(google_book_service),
    );

    let result = service.remove_book_from_offered(&book_id, &user_id).await;

    // Assert
    assert!(result.is_err());
    match result {
        Err(AppError::ValidationError(msg)) => {
            assert_eq!(msg, "Este livro não está na sua lista de possuídos");
        }
        _ => panic!("Erro inesperado"),
    }
} 