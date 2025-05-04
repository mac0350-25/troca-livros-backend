use crate::error::AppError;
use crate::repositories::book_repository::BookRepository;
use crate::repositories::book_repository_test::create_test_book;
use crate::repositories::book_repository_test::setup_test_repository;
use crate::repositories::test_helpers::get_test_mutex;

#[tokio::test]
async fn test_create_book() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;

    // Cria um livro para o teste usando a factory
    let book = create_test_book("abc123", true);

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

    // Busca o livro usando o repository
    let found_book = book_repository
        .find_by_google_id(&book.google_id)
        .await
        .expect("Falha ao buscar livro pelo google_id");

    // Verifica se o livro foi encontrado
    assert!(
        found_book.is_some(),
        "O livro criado não foi encontrado pelo google_id"
    );

    let found_book = found_book.unwrap();

    // Verifica se o ID do livro corresponde ao ID retornado pela criação
    assert_eq!(found_book.id, book_id, "O ID do livro não corresponde ao ID retornado pela criação");

    // Verifica se os dados do livro encontrado correspondem aos dados inseridos
    assert_eq!(found_book.book.google_id, book.google_id);
    assert_eq!(found_book.book.title, book.title);
    assert_eq!(found_book.book.authors, book.authors);
    assert_eq!(found_book.book.publisher, book.publisher);
    assert_eq!(found_book.book.published_date, book.published_date);
    assert_eq!(found_book.book.description, book.description);
    assert_eq!(found_book.book.image_url, book.image_url);
    assert_eq!(found_book.book.page_count, book.page_count);
}

#[tokio::test]
async fn test_create_book_with_invalid_date() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;

    // Cria um livro para o teste usando a factory, com uma data inválida
    let book = create_test_book("abc456", false);

    // Tenta inserir o livro com data inválida e espera um erro de validação
    let result = book_repository.create(&book).await;

    assert!(result.is_err(), "Deveria falhar ao criar livro com data inválida");

    // Verifica se é um erro de validação
    match result {
        Err(AppError::ValidationError(_)) => (),
        _ => panic!("Esperava um erro de validação"),
    }
}
