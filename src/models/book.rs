use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct GoogleBookDto {
    pub google_id: String,
    pub title: String,
    pub authors: Option<String>,
    pub publisher: Option<String>,
    pub published_date: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub page_count: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema, Deserialize)]
pub struct BookSearchRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BookOffered {
    #[schema(value_type = String, format = "uuid")]
    pub book_id: Uuid,
    #[schema(value_type = String, format = "uuid")]
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BookWanted {
    #[schema(value_type = String, format = "uuid")]
    pub book_id: Uuid,
    #[schema(value_type = String, format = "uuid")]
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBookOfferedDto {
    pub book_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBookWantedDto {
    pub book_id: Uuid,
    pub user_id: Uuid,
}

