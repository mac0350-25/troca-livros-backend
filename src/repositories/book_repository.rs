use crate::models::book::GoogleBookDto;
use chrono::NaiveDate;
use sqlx::PgPool;
use async_trait::async_trait;
use uuid::Uuid;
use crate::error::AppError;

// Estender GoogleBookDto para incluir o id do banco de dados
#[derive(Debug, Clone)]
pub struct BookWithId {
    pub id: Uuid,
    pub book: GoogleBookDto,
}

#[async_trait]
pub trait BookRepository: Send + Sync + 'static {
    async fn create(&self, book: &GoogleBookDto) -> Result<Uuid, AppError>;
    async fn find_by_google_id(&self, google_id: &str) -> Result<Option<BookWithId>, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<GoogleBookDto>, AppError>;
    async fn find_by_ids(&self, ids: &[String]) -> Result<Vec<BookWithId>, AppError>;
}

pub struct PgBookRepository {
    pool: PgPool,
}

impl PgBookRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BookRepository for PgBookRepository {
    async fn find_by_google_id(&self, google_id: &str) -> Result<Option<BookWithId>, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT 
                id,
                google_id,
                title,
                author,
                publisher,
                published_date,
                description,
                image_url,
                page_count
            FROM books 
            WHERE google_id = $1
            "#,
            google_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.map(|r| BookWithId {
            id: r.id,
            book: GoogleBookDto {
                google_id: r.google_id.unwrap_or_default(),
                title: r.title,
                authors: Some(r.author),
                publisher: r.publisher,
                published_date: r.published_date.map(|d| d.to_string()),
                description: Some(r.description),
                image_url: Some(r.image_url),
                page_count: r.page_count,
            }
        }))
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<GoogleBookDto>, AppError> {
        // Tenta converter a string em UUID
        let book_id = match Uuid::parse_str(id) {
            Ok(uuid) => uuid,
            Err(_) => return Ok(None), // Retorna None para IDs inválidos
        };

        let result = sqlx::query!(
            r#"
            SELECT 
                google_id,
                title,
                author,
                publisher,
                published_date,
                description,
                image_url,
                page_count
            FROM books 
            WHERE id = $1
            "#,
            book_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(r) => Ok(Some(GoogleBookDto {
                google_id: r.google_id.unwrap_or_default(),
                title: r.title,
                authors: Some(r.author),
                publisher: r.publisher,
                published_date: r.published_date.map(|d| d.to_string()),
                description: Some(r.description),
                image_url: Some(r.image_url),
                page_count: r.page_count,
            })),
            None => Ok(None),
        }
    }

    async fn create(&self, book: &GoogleBookDto) -> Result<Uuid, AppError> {
        // Tenta converter a data de publicação para o formato NaiveDate
        let published_date = match &book.published_date {
            Some(date_str) => {
                match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                    Ok(date) => Some(date),
                    Err(_) => {
                        return Err(AppError::ValidationError(format!(
                            "A data '{}' deve estar no formato AAAA-MM-DD",
                            date_str
                        )));
                    }
                }
            }
            None => None,
        };

        let result = sqlx::query!(
            r#"
            INSERT INTO books (
                title, 
                author, 
                description, 
                image_url, 
                publisher, 
                published_date, 
                page_count, 
                google_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
            &book.title[..book.title.len().min(250)],
            &book.authors.clone().unwrap_or_default()[..book.authors.clone().unwrap_or_default().len().min(250)],
            &book.description.clone().unwrap_or_default()[..book.description.clone().unwrap_or_default().len().min(250)],
            book.image_url.clone().unwrap_or_default(),
            book.publisher,
            published_date,
            book.page_count,
            book.google_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.id)
    }

    async fn find_by_ids(&self, ids: &[String]) -> Result<Vec<BookWithId>, AppError> {
        // Validar e converter as strings para UUIDs
        let mut valid_uuids = Vec::with_capacity(ids.len());
        
        for id in ids {
            match Uuid::parse_str(id) {
                Ok(uuid) => valid_uuids.push(uuid),
                Err(_) => continue, // Ignora IDs inválidos
            }
        }
        
        // Se não há UUIDs válidos, retorna uma lista vazia
        if valid_uuids.is_empty() {
            return Ok(Vec::new());
        }

        // Consulta para buscar livros com os IDs fornecidos
        let result = sqlx::query!(
            r#"
            SELECT 
                id,
                google_id,
                title,
                author,
                publisher,
                published_date,
                description,
                image_url,
                page_count
            FROM books 
            WHERE id = ANY($1)
            "#,
            &valid_uuids[..]
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Mapear os resultados para BookWithId
        let books = result
            .into_iter()
            .map(|r| BookWithId {
                id: r.id,
                book: GoogleBookDto {
                    google_id: r.google_id.unwrap_or_default(),
                    title: r.title,
                    authors: Some(r.author),
                    publisher: r.publisher,
                    published_date: r.published_date.map(|d| d.to_string()),
                    description: Some(r.description),
                    image_url: Some(r.image_url),
                    page_count: r.page_count,
                }
            })
            .collect();

        Ok(books)
    }
}
