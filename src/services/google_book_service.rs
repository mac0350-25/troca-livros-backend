use crate::error::AppError;
use crate::models::book::GoogleBookDto;
use reqwest::Client;
use serde_json::Value;
pub trait GoogleBookService: Send + Sync {
    fn search_books<'a>(
        &'a self,
        query: &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<GoogleBookDto>, AppError>> + Send + 'a>,
    >;
}

pub struct GoogleBookServiceImpl {
    client: Client,
}

impl GoogleBookServiceImpl {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl GoogleBookService for GoogleBookServiceImpl {
    fn search_books<'a>(
        &'a self,
        query: &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<GoogleBookDto>, AppError>> + Send + 'a>,
    > {
        Box::pin(async move {
            let mut url = String::from("https://www.googleapis.com/books/v1/volumes?q=");
            url.push_str(&query);
            url.push_str("&fields=items(id,volumeInfo(title,authors,publisher,publishedDate,description,pageCount,imageLinks/thumbnail))");

            let response = self.client.get(&url).send().await.map_err(|e| {
                AppError::InternalServerError(format!("Erro ao buscar livros: {}", e))
            })?;

            let data: Value = response.json().await.map_err(|e| {
                AppError::InternalServerError(format!("Erro ao processar resposta: {}", e))
            })?;

            let items = match data.get("items") {
                Some(items) => items,
                None => return Ok(vec![]),
            };

            let mut books = Vec::new();

            if let Some(items_array) = items.as_array() {
                for item in items_array {
                    let google_id = item["id"].as_str().unwrap_or_default().to_string();

                    let volume_info = &item["volumeInfo"];

                    // Extrair informações
                    let title = volume_info["title"].as_str();
                    let title = title.unwrap_or_default().to_string();

                    // Processar autores
                    let authors = match volume_info["authors"].as_array() {
                        Some(authors_array) => {
                            let mut authors_vec: Vec<String> = Vec::new();
                            for author in authors_array {
                                if let Some(author_str) = author.as_str() {
                                    authors_vec.push(author_str.to_string());
                                }
                            }
                            Some(authors_vec.join(", "))
                        }
                        None => None,
                    };

                    let publisher = volume_info["publisher"].as_str().map(|s| s.to_string());

                    let published_date = match volume_info["publishedDate"].as_str() {
                        Some(date_str) => Some(date_str.to_string()),
                        None => None,
                    };

                    let description = volume_info["description"].as_str().map(|s| s.to_string());
                    let page_count = volume_info["pageCount"].as_i64().map(|n| n as i32);

                    let image_url = match volume_info.get("imageLinks") {
                        Some(image_links) => match image_links.get("thumbnail") {
                            Some(thumbnail) => thumbnail.as_str().map(|s| s.to_string()),
                            None => None,
                        },
                        None => None,
                    };

                    books.push(GoogleBookDto {
                        google_id,
                        title,
                        authors,
                        publisher,
                        published_date,
                        description,
                        image_url,
                        page_count,
                    });
                }
            }

            Ok(books)
        })
    }
}
