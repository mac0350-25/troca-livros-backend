use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::book::{BookOffered, CreateBookOfferedDto};

#[async_trait]
pub trait BooksOfferedRepository: Send + Sync + 'static {
    async fn create(&self, book_offered: &CreateBookOfferedDto) -> Result<BookOffered, AppError>;
    async fn find(&self, book_id: &Uuid, user_id: &Uuid) -> Result<Option<BookOffered>, AppError>;
    async fn delete(&self, book_id: &Uuid, user_id: &Uuid) -> Result<bool, AppError>;
}

pub struct PgBooksOfferedRepository {
    pool: PgPool,
}

impl PgBooksOfferedRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BooksOfferedRepository for PgBooksOfferedRepository {
    async fn create(&self, book_offered: &CreateBookOfferedDto) -> Result<BookOffered, AppError> {
        // Inserir na tabela books_offered
        let result = sqlx::query!(
            r#"
            INSERT INTO books_offered (book_id, user_id)
            VALUES ($1, $2)
            RETURNING book_id, user_id
            "#,
            book_offered.book_id,
            book_offered.user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                AppError::ValidationError("Este livro já está na sua lista de possuídos".to_string())
            } else if e.to_string().contains("foreign key constraint") {
                if e.to_string().contains("books_offered_book_id_fkey") {
                    AppError::ValidationError(format!(
                        "Livro com ID {} não encontrado",
                        book_offered.book_id
                    ))
                } else if e.to_string().contains("books_offered_user_id_fkey") {
                    AppError::ValidationError(format!(
                        "Usuário com ID {} não encontrado",
                        book_offered.user_id
                    ))
                } else {
                    AppError::DatabaseError(e.to_string())
                }
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(BookOffered {
            book_id: result.book_id,
            user_id: result.user_id,
        })
    }

    async fn find(&self, book_id: &Uuid, user_id: &Uuid) -> Result<Option<BookOffered>, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT book_id, user_id
            FROM books_offered
            WHERE book_id = $1 AND user_id = $2
            "#,
            book_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.map(|r| BookOffered {
            book_id: r.book_id,
            user_id: r.user_id,
        }))
    }

    async fn delete(&self, book_id: &Uuid, user_id: &Uuid) -> Result<bool, AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM books_offered
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