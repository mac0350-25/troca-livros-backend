use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::book::{BookWanted, CreateBookWantedDto};

#[async_trait]
pub trait BooksWantedRepository: Send + Sync + 'static {
    async fn create(&self, book_wanted: &CreateBookWantedDto) -> Result<BookWanted, AppError>;
    async fn find(&self, book_id: &Uuid, user_id: &Uuid) -> Result<Option<BookWanted>, AppError>;
    async fn delete(&self, book_id: &Uuid, user_id: &Uuid) -> Result<bool, AppError>;
}

pub struct PgBooksWantedRepository {
    pool: PgPool,
}

impl PgBooksWantedRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BooksWantedRepository for PgBooksWantedRepository {
    async fn create(&self, book_wanted: &CreateBookWantedDto) -> Result<BookWanted, AppError> {
        // Inserir na tabela books_wanted
        let result = sqlx::query!(
            r#"
            INSERT INTO books_wanted (book_id, user_id)
            VALUES ($1, $2)
            RETURNING book_id, user_id
            "#,
            book_wanted.book_id,
            book_wanted.user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                AppError::ValidationError("Este livro já está na sua lista de desejados".to_string())
            } else if e.to_string().contains("foreign key constraint") {
                if e.to_string().contains("books_wanted_book_id_fkey") {
                    AppError::ValidationError(format!(
                        "Livro com ID {} não encontrado",
                        book_wanted.book_id
                    ))
                } else if e.to_string().contains("books_wanted_user_id_fkey") {
                    AppError::ValidationError(format!(
                        "Usuário com ID {} não encontrado",
                        book_wanted.user_id
                    ))
                } else {
                    AppError::DatabaseError(e.to_string())
                }
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(BookWanted {
            book_id: result.book_id,
            user_id: result.user_id,
        })
    }

    async fn find(&self, book_id: &Uuid, user_id: &Uuid) -> Result<Option<BookWanted>, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT book_id, user_id
            FROM books_wanted
            WHERE book_id = $1 AND user_id = $2
            "#,
            book_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.map(|r| BookWanted {
            book_id: r.book_id,
            user_id: r.user_id,
        }))
    }

    async fn delete(&self, book_id: &Uuid, user_id: &Uuid) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM books_wanted
            WHERE book_id = $1 AND user_id = $2
            "#,
            book_id,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Retorna true se algo foi excluído, false caso contrário
        Ok(result.rows_affected() > 0)
    }
} 