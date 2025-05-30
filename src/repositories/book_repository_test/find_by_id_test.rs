use crate::repositories::book_repository::BookRepository;
use crate::repositories::book_repository_test::create_test_book;
use crate::repositories::book_repository_test::setup_test_repository;
use crate::repositories::test_helpers::get_test_mutex;

#[tokio::test]
async fn test_find_by_id() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;

    // Cria um livro para o teste usando a factory
    let book = create_test_book("test_find_123", true);

    // Insere o livro no banco de dados
    let book_id = book_repository
        .create(&book)
        .await
        .expect("Falha ao criar livro");

    // Verifica se o ID retornado não é vazio
    assert!(
        book_id.to_string().len() > 0,
        "O ID do livro não deve ser vazio"
    );

    // Busca o livro usando o repository - convertendo UUID para string
    let found_book = book_repository
        .find_by_id(&book_id.to_string())
        .await
        .expect("Falha ao buscar livro pelo ID");

    // Verifica se o livro foi encontrado
    assert!(
        found_book.is_some(),
        "O livro criado não foi encontrado pelo ID"
    );

    let found_book = found_book.unwrap();

    // Verifica se os dados do livro encontrado correspondem aos dados inseridos
    assert_eq!(found_book.google_id, book.google_id);
    assert_eq!(found_book.title, book.title);
    assert_eq!(found_book.authors, book.authors);
    assert_eq!(found_book.publisher, book.publisher);
    assert_eq!(found_book.published_date, book.published_date);
    assert_eq!(found_book.description, book.description);
    assert_eq!(found_book.image_url, book.image_url);
    assert_eq!(found_book.page_count, book.page_count);
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;

    // Busca um livro com um ID que não existe
    let result = book_repository
        .find_by_id("nonexistent_id")
        .await
        .expect("Falha ao buscar livro pelo ID");

    // Verifica se o resultado é None
    assert!(
        result.is_none(),
        "Não deveria encontrar um livro com ID inexistente"
    );
}
