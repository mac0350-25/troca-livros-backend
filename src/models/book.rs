use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Serialize, ToSchema)]
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
