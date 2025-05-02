use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::AppError;
use crate::repositories::book_repository::{BookRepository, BookWithId};
use crate::repositories::books_offered_repository::BooksOfferedRepository;
use crate::repositories::books_wanted_repository::BooksWantedRepository;

#[derive(Debug)]
pub struct UserBooks {
    pub offered_books: Vec<BookWithId>,
    pub wanted_books: Vec<BookWithId>,
}

#[async_trait]
pub trait BookService: Send + Sync + 'static {
    async fn get_user_books(&self, user_id: &Uuid) -> Result<UserBooks, AppError>;
}

pub struct BookServiceImpl {
    book_repository: Arc<dyn BookRepository>,
    books_offered_repository: Arc<dyn BooksOfferedRepository>,
    books_wanted_repository: Arc<dyn BooksWantedRepository>,
}

impl BookServiceImpl {
    pub fn new(
        book_repository: Arc<dyn BookRepository>,
        books_offered_repository: Arc<dyn BooksOfferedRepository>,
        books_wanted_repository: Arc<dyn BooksWantedRepository>,
    ) -> Self {
        Self {
            book_repository,
            books_offered_repository,
            books_wanted_repository,
        }
    }
}

#[async_trait]
impl BookService for BookServiceImpl {
    async fn get_user_books(&self, user_id: &Uuid) -> Result<UserBooks, AppError> {
        // Obter IDs de livros possuídos pelo usuário
        let offered_book_ids = self.books_offered_repository.find_by_user_id(user_id).await?;
        
        // Obter IDs de livros desejados pelo usuário
        let wanted_book_ids = self.books_wanted_repository.find_by_user_id(user_id).await?;
        
        // Converter IDs para strings para usar no find_by_ids
        let offered_ids: Vec<String> = offered_book_ids.iter().map(|id| id.to_string()).collect();
        let wanted_ids: Vec<String> = wanted_book_ids.iter().map(|id| id.to_string()).collect();
        
        // Buscar detalhes dos livros possuídos
        let offered_books = if !offered_ids.is_empty() {
            self.book_repository.find_by_ids(&offered_ids).await?
        } else {
            Vec::new()
        };
        
        // Buscar detalhes dos livros desejados
        let wanted_books = if !wanted_ids.is_empty() {
            self.book_repository.find_by_ids(&wanted_ids).await?
        } else {
            Vec::new()
        };
        
        Ok(UserBooks {
            offered_books,
            wanted_books,
        })
    }
} 