use std::sync::Arc;

use mockall::predicate::*;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::book::BookWanted;
use crate::services::book_wanted_service::{BookWantedService, BookWantedServiceImpl};
use crate::services::book_offered_wanted_service_test::{MockBookRepository, MockBooksWantedRepository, MockBooksOfferedRepository, MockGoogleBookService};

#[tokio::test]
async fn test_remove_book_from_wanted() {
    // Arrange
    let book_repo = MockBookRepository::new();
    let mut books_wanted_repo = MockBooksWantedRepository::new();
    let books_offered_repo = MockBooksOfferedRepository::new();
    let google_book_service = MockGoogleBookService::new();

    let book_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    // Configurar o mock do books_wanted_repository para retornar Some para find
    books_wanted_repo
        .expect_find()
        .with(eq(book_id), eq(user_id))
        .times(1)
        .returning(move |book_id, user_id| {
            Ok(Some(BookWanted {
                book_id: *book_id,
                user_id: *user_id,
            }))
        });

    // Configurar o mock do books_wanted_repository para retornar true para delete
    books_wanted_repo
        .expect_delete()
        .with(eq(book_id), eq(user_id))
        .times(1)
        .returning(|_, _| Ok(true));

    // Act
    let service = BookWantedServiceImpl::new(
        Arc::new(book_repo),
        Arc::new(books_wanted_repo),
        Arc::new(books_offered_repo),
        Arc::new(google_book_service),
    );

    let result = service.remove_book_from_wanted(&book_id, &user_id).await;

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[tokio::test]
async fn test_remove_book_from_wanted_when_not_wanted() {
    // Arrange
    let book_repo = MockBookRepository::new();
    let mut books_wanted_repo = MockBooksWantedRepository::new();
    let books_offered_repo = MockBooksOfferedRepository::new();
    let google_book_service = MockGoogleBookService::new();

    let book_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    // Configurar o mock do books_wanted_repository para retornar None para find
    books_wanted_repo
        .expect_find()
        .with(eq(book_id), eq(user_id))
        .times(1)
        .returning(|_, _| Ok(None));

    // Act
    let service = BookWantedServiceImpl::new(
        Arc::new(book_repo),
        Arc::new(books_wanted_repo),
        Arc::new(books_offered_repo),
        Arc::new(google_book_service),
    );

    let result = service.remove_book_from_wanted(&book_id, &user_id).await;

    // Assert
    assert!(result.is_err());
    match result {
        Err(AppError::ValidationError(msg)) => {
            assert_eq!(msg, "Este livro não está na sua lista de desejados");
        }
        _ => panic!("Erro inesperado"),
    }
} 