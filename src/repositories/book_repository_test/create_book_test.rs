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
async fn test_create_book_with_invalid_date() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;

    // Cria um livro com data em formato inválido usando a factory
    let book = create_test_book(
        "invalid_date_test",
        false,
    );

    // Tenta inserir o livro e espera um erro de formato de data
    let result = book_repository.create(&book).await;

    // Verifica se o resultado é um erro e do tipo esperado
    match result {
        Err(AppError::ValidationError(error_msg)) => {
            // Verifica se a mensagem de erro contém a informação sobre o formato esperado
            assert!(error_msg.contains("deve estar no formato AAAA-MM-DD"));
            assert!(error_msg.contains("10/05/2022"));
            println!("Erro de validação capturado com sucesso: {}", error_msg);
        }
        Ok(_) => panic!("O teste deveria falhar com erro de formato de data"),
        Err(e) => panic!("Erro inesperado: {:?}", e),
    }

    // Verifica se o livro não foi inserido no banco utilizando o repository
    let not_found = book_repository
        .find_by_google_id(&book.google_id)
        .await
        .expect("Falha na consulta");

    // Confirma que o livro não existe no banco
    assert!(
        not_found.is_none(),
        "O livro com data inválida não deveria ter sido inserido"
    );
}
