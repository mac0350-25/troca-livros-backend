pub mod add_book_to_offered_test;
pub mod remove_book_from_offered_test;
pub mod add_book_to_wanted_test;
pub mod remove_book_from_wanted_test;

use mockall::{mock, predicate::*};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::book::{BookOffered, CreateBookOfferedDto, GoogleBookDto, BookWanted};
use crate::repositories::book_repository::BookWithId;

// Mock para o BookRepository
mock! {
    pub BookRepository {}

    #[async_trait::async_trait]
    impl crate::repositories::book_repository::BookRepository for BookRepository {
        async fn create(&self, book: &GoogleBookDto) -> Result<Uuid, AppError>;
        async fn find_by_google_id(&self, google_id: &str) -> Result<Option<BookWithId>, AppError>;
        async fn find_by_id(&self, id: &str) -> Result<Option<GoogleBookDto>, AppError>;
    }
}

// Mock para o BooksOfferedRepository
mock! {
    pub BooksOfferedRepository {}

    #[async_trait::async_trait]
    impl crate::repositories::books_offered_repository::BooksOfferedRepository for BooksOfferedRepository {
        async fn create(&self, book_offered: &CreateBookOfferedDto) -> Result<BookOffered, AppError>;
        async fn find(&self, book_id: &Uuid, user_id: &Uuid) -> Result<Option<BookOffered>, AppError>;
        async fn delete(&self, book_id: &Uuid, user_id: &Uuid) -> Result<bool, AppError>;
        async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Uuid>, AppError>;
    }
}

// Mock para o BooksWantedRepository
mock! {
    pub BooksWantedRepository {}

    #[async_trait::async_trait]
    impl crate::repositories::books_wanted_repository::BooksWantedRepository for BooksWantedRepository {
        async fn create(&self, book_wanted: &crate::models::book::CreateBookWantedDto) -> Result<BookWanted, AppError>;
        async fn find(&self, book_id: &Uuid, user_id: &Uuid) -> Result<Option<BookWanted>, AppError>;
        async fn delete(&self, book_id: &Uuid, user_id: &Uuid) -> Result<bool, AppError>;
    }
}

// Mock para o GoogleBookService - versão simplificada
// O problema era com os lifetimes na trait original
pub struct MockGoogleBookService {
    pub find_book_by_id_fn: Box<dyn Fn(&str) -> Result<GoogleBookDto, AppError> + Send + Sync>,
}

impl MockGoogleBookService {
    pub fn new() -> Self {
        // Por padrão, retorna um erro (será substituído nos testes)
        Self {
            find_book_by_id_fn: Box::new(|_| Err(AppError::NotFoundError("Livro não encontrado".to_string()))),
        }
    }

    pub fn with_find_book_by_id<F>(mut self, f: F) -> Self 
    where 
        F: Fn(&str) -> Result<GoogleBookDto, AppError> + 'static + Send + Sync 
    {
        self.find_book_by_id_fn = Box::new(f);
        self
    }
}

impl crate::services::google_book_service::GoogleBookService for MockGoogleBookService {
    fn search_books<'a>(
        &'a self,
        _query: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<GoogleBookDto>, AppError>> + Send + 'a>> {
        // Podemos implementar se for necessário nos testes
        Box::pin(async { Ok(vec![]) })
    }

    fn find_book_by_id<'a>(
        &'a self,
        google_id: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<GoogleBookDto, AppError>> + Send + 'a>> {
        let result = (self.find_book_by_id_fn)(google_id);
        Box::pin(async move { result })
    }
} 