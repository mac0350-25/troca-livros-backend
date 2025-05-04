pub mod create_book_test;
pub mod find_book_by_google_id_test;
pub mod find_by_google_id_test;
pub mod find_by_id_test;
pub mod find_by_ids_test;

use crate::models::book::GoogleBookDto;
use crate::repositories::book_repository::PgBookRepository;
use crate::repositories::test_helpers::get_test_db_pool;


async fn setup_test_repository() -> PgBookRepository {
    // Obtém o pool de conexão com o banco de dados de teste
    let pool = get_test_db_pool().await;

    // Limpa o banco de dados para garantir o isolamento dos testes
    crate::repositories::test_helpers::clean_database(&pool).await;

    // Criamos o DatabasePool com o pool real
    PgBookRepository::new(pool)
}

pub fn create_test_book(google_id: &str, with_valid_date: bool) -> GoogleBookDto {
    GoogleBookDto {
        google_id: String::from(google_id),
        title: String::from("Livro de Teste"),
        authors: Some(String::from("Autor Teste")),
        publisher: Some(String::from("Editora Teste")),
        // Usa data válida ou inválida conforme solicitado
        published_date: Some(String::from(if with_valid_date {
            "2022-05-10"
        } else {
            "10/05/2022"
        })),
        description: Some(String::from("Esta é uma descrição de teste")),
        image_url: Some(String::from("http://example.com/livro.jpg")),
        page_count: Some(300),
    }
}
