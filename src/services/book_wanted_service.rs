use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::book::{BookWanted, CreateBookWantedDto};
use crate::repositories::book_repository::BookRepository;
use crate::repositories::books_wanted_repository::BooksWantedRepository;
use crate::repositories::books_offered_repository::BooksOfferedRepository;
use crate::services::google_book_service::GoogleBookService;

#[async_trait]
pub trait BookWantedService: Send + Sync + 'static {
    async fn add_book_to_wanted(&self, google_id: &str, user_id: &Uuid) -> Result<BookWanted, AppError>;
    async fn remove_book_from_wanted(&self, book_id: &Uuid, user_id: &Uuid) -> Result<bool, AppError>;
}

pub struct BookWantedServiceImpl {
    book_repository: Arc<dyn BookRepository>,
    books_wanted_repository: Arc<dyn BooksWantedRepository>,
    books_offered_repository: Arc<dyn BooksOfferedRepository>,
    google_book_service: Arc<dyn GoogleBookService>,
}

impl BookWantedServiceImpl {
    pub fn new(
        book_repository: Arc<dyn BookRepository>,
        books_wanted_repository: Arc<dyn BooksWantedRepository>,
        books_offered_repository: Arc<dyn BooksOfferedRepository>,
        google_book_service: Arc<dyn GoogleBookService>,
    ) -> Self {
        Self {
            book_repository,
            books_wanted_repository,
            books_offered_repository,
            google_book_service,
        }
    }
}

#[async_trait]
impl BookWantedService for BookWantedServiceImpl {
    async fn add_book_to_wanted(&self, google_id: &str, user_id: &Uuid) -> Result<BookWanted, AppError> {
        // Variável para armazenar o UUID do banco de dados
        let book_uuid: Uuid;
        
        // Verificar se o livro existe no banco de dados e obter seu ID interno
        let existing_book = self.book_repository.find_by_google_id(google_id).await?;
        
        if let Some(book_with_id) = existing_book {
            // Se o livro já existe, usar o ID existente
            book_uuid = book_with_id.id;
        } else {
            // Livro não existe, precisa ser criado
            // Buscar do Google Books API
            let book_dto = self.google_book_service.find_book_by_id(google_id).await?;
            
            // Criar o livro no banco de dados
            book_uuid = self.book_repository.create(&book_dto).await?;
        }
        
        // Verificar se o livro já está na lista de desejados do usuário
        if let Some(_) = self.books_wanted_repository.find(&book_uuid, user_id).await? {
            return Err(AppError::ValidationError("Este livro já está na sua lista de desejados".to_string()));
        }
        
        // Verificar se o livro já está na lista de possuídos do usuário
        if let Some(_) = self.books_offered_repository.find(&book_uuid, user_id).await? {
            return Err(AppError::ValidationError("Este livro já está na sua lista de possuídos".to_string()));
        }
        
        // Criar DTO para adicionar à lista de desejados
        let create_dto = CreateBookWantedDto {
            book_id: book_uuid,
            user_id: *user_id,
        };

        // Adicionar à lista de livros desejados
        let book_wanted = self.books_wanted_repository.create(&create_dto).await?;

        Ok(book_wanted)
    }

    async fn remove_book_from_wanted(&self, book_id: &Uuid, user_id: &Uuid) -> Result<bool, AppError> {
        // Verificar se o livro existe na lista de desejados do usuário
        let exists = self.books_wanted_repository.find(book_id, user_id).await?;
        if exists.is_none() {
            return Err(AppError::ValidationError(
                "Este livro não está na sua lista de desejados".to_string(),
            ));
        }

        // Remover da lista de livros desejados
        self.books_wanted_repository.delete(book_id, user_id).await
    }
} 