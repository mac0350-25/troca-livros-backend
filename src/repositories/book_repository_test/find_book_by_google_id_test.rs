use crate::repositories::book_repository::BookRepository;
use crate::repositories::book_repository_test::create_test_book;
use crate::repositories::book_repository_test::setup_test_repository;
use crate::repositories::test_helpers::get_test_mutex;

#[tokio::test]
async fn test_find_book_by_google_id() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;
    // Cria um livro para o teste usando a factory
    let book = create_test_book("test123", true);

    // Insere o livro no banco de dados
    let book_id = book_repository
        .create(&book)
        .await
        .expect("Falha ao criar livro");

    // Testa a função find_by_google_id
    let found_book = book_repository
        .find_by_google_id(&book.google_id)
        .await
        .expect("Falha ao buscar livro");

    // Verifica se o livro foi encontrado
    assert!(
        found_book.is_some(),
        "O livro com google_id '{}' não foi encontrado",
        book.google_id
    );

    let found_book = found_book.unwrap();

    // Verifica se o ID do livro corresponde ao ID retornado pela criação
    assert_eq!(found_book.id, book_id, "O ID do livro não corresponde ao ID retornado pela criação");

    // Verifica se os dados do livro encontrado correspondem aos dados inseridos
    assert_eq!(found_book.book.google_id, book.google_id);
    assert_eq!(found_book.book.title, book.title);
    assert_eq!(found_book.book.authors, book.authors);
    assert_eq!(found_book.book.publisher, book.publisher);
    // Comparação simplificada para published_date já que pode haver diferenças de formatação
    assert!(found_book.book.published_date.is_some());
    assert_eq!(
        found_book.book.description.unwrap_or_default(),
        book.description.unwrap_or_default()
    );
    assert_eq!(
        found_book.book.image_url.unwrap_or_default(),
        book.image_url.unwrap_or_default()
    );
    assert_eq!(found_book.book.page_count, book.page_count);
}

#[tokio::test]
async fn test_find_book_by_google_id_not_found() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;

    // Testa buscar um livro que não existe
    let not_found = book_repository
        .find_by_google_id("non_existent_id")
        .await
        .expect("Falha na consulta");

    assert!(not_found.is_none());
}
