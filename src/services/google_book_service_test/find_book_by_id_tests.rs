use crate::services::google_book_service::{GoogleBookService, GoogleBookServiceImpl};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn find_book_by_google_id_test() {
        // Arrange
        let service = GoogleBookServiceImpl::new();
        let google_id = "y1FgDwAAQBAJ"; // ID do livro "Primeiros passos com a linguagem Rust"

        // Act
        let result = service.find_book_by_id(google_id).await;

        // Assert
        assert!(result.is_ok(), "A busca por ID deveria ser bem-sucedida");

        let book = result.unwrap();

        // Verificamos os dados específicos do livro
        assert_eq!(
            book.google_id, google_id,
            "O ID do Google deve corresponder"
        );
        assert!(
            book.title.contains("Rust"),
            "O título do livro deve conter 'Rust'"
        );

        // Verificamos se os campos opcionais possuem estrutura adequada quando presentes
        if let Some(authors) = &book.authors {
            assert!(
                !authors.is_empty(),
                "Autores não devem estar vazios quando presentes"
            );
        }

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

        if let Some(image_url) = &book.image_url {
            assert!(
                image_url.starts_with("http"),
                "URL da imagem deve ser uma URL válida quando presente"
            );
        }
    }

    #[tokio::test]
    async fn find_book_by_nonexistent_id_test() {
        // Arrange
        let service = GoogleBookServiceImpl::new();
        let google_id = "idquenaoexiste123456789"; // ID inexistente

        // Act
        let result = service.find_book_by_id(google_id).await;

        // Assert
        assert!(
            result.is_err(),
            "A busca por ID inexistente deve retornar erro"
        );
    }
}
