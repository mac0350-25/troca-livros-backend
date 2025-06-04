use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::trade::PossibleTrade;
use crate::models::book::GoogleBookDto;
use crate::models::user::UserResponse;

#[async_trait]
pub trait TradeRepository: Send + Sync + 'static {
    async fn find_possible_trades(&self, user_id: Uuid) -> Result<Vec<PossibleTrade>, AppError>;
}

pub struct PgTradeRepository {
    pool: PgPool,
}

impl PgTradeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TradeRepository for PgTradeRepository {
    async fn find_possible_trades(&self, user_id: Uuid) -> Result<Vec<PossibleTrade>, AppError> {
        // Query complexa que encontra trocas possíveis:
        // 1. Pega livros que o usuário oferece
        // 2. Encontra outros usuários que querem esses livros
        // 3. Verifica se o usuário atual quer algum livro que esses outros usuários oferecem
        let result = sqlx::query!(
            r#"
            SELECT DISTINCT
                -- Livro que o usuário oferece
                offered_book.id as offered_book_id,
                offered_book.title as offered_book_title,
                offered_book.author as offered_book_author,
                offered_book.publisher as offered_book_publisher,
                offered_book.published_date as offered_book_published_date,
                offered_book.description as offered_book_description,
                offered_book.image_url as offered_book_image_url,
                offered_book.page_count as offered_book_page_count,
                offered_book.google_id as offered_book_google_id,
                
                -- Livro que o usuário quer (oferecido pelo parceiro)
                wanted_book.id as wanted_book_id,
                wanted_book.title as wanted_book_title,
                wanted_book.author as wanted_book_author,
                wanted_book.publisher as wanted_book_publisher,
                wanted_book.published_date as wanted_book_published_date,
                wanted_book.description as wanted_book_description,
                wanted_book.image_url as wanted_book_image_url,
                wanted_book.page_count as wanted_book_page_count,
                wanted_book.google_id as wanted_book_google_id,
                
                -- Parceiro de troca
                partner.id as partner_id,
                partner.name as partner_name,
                partner.email as partner_email,
                partner.created_at as partner_created_at,
                partner.updated_at as partner_updated_at
            FROM 
                -- Livros que o usuário oferece
                books_offered my_offers
                INNER JOIN books offered_book ON my_offers.book_id = offered_book.id
                
                -- Outros usuários que querem esses livros
                INNER JOIN books_wanted partner_wants ON partner_wants.book_id = offered_book.id
                INNER JOIN users partner ON partner_wants.user_id = partner.id
                
                -- Livros que esses outros usuários oferecem
                INNER JOIN books_offered partner_offers ON partner_offers.user_id = partner.id
                INNER JOIN books wanted_book ON partner_offers.book_id = wanted_book.id
                
                -- O usuário atual quer esses livros
                INNER JOIN books_wanted my_wants ON my_wants.book_id = wanted_book.id
            WHERE 
                my_offers.user_id = $1
                AND my_wants.user_id = $1
                AND partner.id != $1
            ORDER BY 
                partner.name, offered_book.title, wanted_book.title
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let trades = result
            .into_iter()
            .map(|row| PossibleTrade {
                offered_book_id: row.offered_book_id,
                offered_book: GoogleBookDto {
                    google_id: row.offered_book_google_id.unwrap_or_default(),
                    title: row.offered_book_title,
                    authors: Some(row.offered_book_author),
                    publisher: row.offered_book_publisher,
                    published_date: row.offered_book_published_date.map(|d| d.to_string()),
                    description: Some(row.offered_book_description),
                    image_url: Some(row.offered_book_image_url),
                    page_count: row.offered_book_page_count,
                },
                wanted_book_id: row.wanted_book_id,
                wanted_book: GoogleBookDto {
                    google_id: row.wanted_book_google_id.unwrap_or_default(),
                    title: row.wanted_book_title,
                    authors: Some(row.wanted_book_author),
                    publisher: row.wanted_book_publisher,
                    published_date: row.wanted_book_published_date.map(|d| d.to_string()),
                    description: Some(row.wanted_book_description),
                    image_url: Some(row.wanted_book_image_url),
                    page_count: row.wanted_book_page_count,
                },
                trade_partner: UserResponse {
                    id: row.partner_id,
                    name: row.partner_name,
                    email: row.partner_email,
                    created_at: row.partner_created_at,
                    updated_at: row.partner_updated_at,
                },
            })
            .collect();

        Ok(trades)
    }
} 