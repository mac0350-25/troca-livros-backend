use crate::services::google_book_service::{GoogleBookService, GoogleBookServiceImpl};
use crate::services::http_service::HttpServiceImpl;
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_books_real_api() {
        // Arrange
        let http_service = Arc::new(HttpServiceImpl::new());
        let service = GoogleBookServiceImpl::new(http_service);
        let query = "Rust Programming Language";

        // Act
        let result = service.search_books(query).await;

        // Assert
        assert!(result.is_ok(), "A busca de livros deveria ser bem-sucedida");

        let books = result.unwrap();
        assert!(
            !books.is_empty(),
            "A busca deveria retornar pelo menos um livro"
        );

        // Verificar estrutura dos dados retornados no primeiro livro
        let first_book = &books[0];
        assert!(
            !first_book.google_id.is_empty(),
            "O ID do Google não deve estar vazio"
        );
        assert!(
            !first_book.title.is_empty(),
            "O título não deve estar vazio"
        );

        // Verificamos se os campos opcionais possuem estrutura adequada quando presentes
        if let Some(authors) = &first_book.authors {
            assert!(
                !authors.is_empty(),
                "Autores não devem estar vazios quando presentes"
            );
        }

        if let Some(image_url) = &first_book.image_url {
            assert!(
                image_url.starts_with("http"),
                "URL da imagem deve ser uma URL válida quando presente"
            );
        }

        if let Some(page_count) = first_book.page_count {
            assert!(
                page_count > 0,
                "Contagem de páginas deve ser positiva quando presente"
            );
        }
    }

    #[tokio::test]
    async fn test_search_books_with_specific_author() {
        // Arrange
        let http_service = Arc::new(HttpServiceImpl::new());
        let service = GoogleBookServiceImpl::new(http_service);
        let query = "author:Martin Fowler";

        // Act
        let result = service.search_books(query).await;

        // Assert
        assert!(result.is_ok(), "A busca por autor deveria ser bem-sucedida");

        let books = result.unwrap();
        if !books.is_empty() {
            // Se houver resultados, verificar se o autor está presente em pelo menos um livro
            let author_found = books.iter().any(|book| {
                if let Some(authors) = &book.authors {
                    authors.contains("Fowler")
                } else {
                    false
                }
            });

            assert!(
                author_found,
                "Pelo menos um livro deveria conter o autor buscado"
            );
        }
    }

    #[tokio::test]
    async fn test_search_books_with_nonexistent_title() {
        // Arrange
        let http_service = Arc::new(HttpServiceImpl::new());
        let service = GoogleBookServiceImpl::new(http_service);
        // Uma string improvável de corresponder a um título real
        let query = "título extremamente improvável de existir 9283749232874";

        // Act
        let result = service.search_books(query).await;

        // Assert
        assert!(
            result.is_ok(),
            "A busca por título inexistente deve ser processada sem erro"
        );

        let books = result.unwrap();
        assert!(
            books.is_empty(),
            "A busca por título inexistente deve retornar lista vazia"
        );
    }

    #[tokio::test]
    async fn test_search_books_response_structure() {
        // Arrange
        let http_service = Arc::new(HttpServiceImpl::new());
        let service = GoogleBookServiceImpl::new(http_service);
        // Um livro bem conhecido que deve ter todos os campos preenchidos
        let query = "title:Clean Code Robert Martin";

        // Act
        let result = service.search_books(query).await;

        // Assert
        assert!(result.is_ok(), "A busca deveria ser bem-sucedida");

        let books = result.unwrap();
        if !books.is_empty() {
            let book = &books[0];

            // Verificamos a estrutura completa do objeto retornado
            assert!(
                !book.google_id.is_empty(),
                "ID do Google deve estar preenchido"
            );
            assert!(!book.title.is_empty(), "Título deve estar preenchido");

            // Campos opcionais devem ter estrutura válida se presentes
            if let Some(publisher) = &book.publisher {
                assert!(
                    !publisher.is_empty(),
                    "Editora não deve estar vazia quando presente"
                );
            }

            if let Some(published_date) = &book.published_date {
                assert!(
                    !published_date.is_empty(),
                    "Data de publicação não deve estar vazia quando presente"
                );
            }

            if let Some(description) = &book.description {
                assert!(
                    !description.is_empty(),
                    "Descrição não deve estar vazia quando presente"
                );
            }
        }
    }
}
