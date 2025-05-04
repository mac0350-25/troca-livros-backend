use crate::repositories::book_repository::BookRepository;
use crate::repositories::book_repository_test::create_test_book;
use crate::repositories::book_repository_test::setup_test_repository;
use crate::repositories::test_helpers::get_test_mutex;
use uuid::Uuid;

#[tokio::test]
async fn test_find_by_ids_multiple_books() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;

    // Cria múltiplos livros para o teste
    let book1 = create_test_book("test_find_multiple_1", true);
    let book2 = create_test_book("test_find_multiple_2", true);
    let book3 = create_test_book("test_find_multiple_3", true);

    // Insere os livros no banco de dados
    let book_id1 = book_repository
        .create(&book1)
        .await
        .expect("Falha ao criar livro 1");
    
    let book_id2 = book_repository
        .create(&book2)
        .await
        .expect("Falha ao criar livro 2");
    
    let book_id3 = book_repository
        .create(&book3)
        .await
        .expect("Falha ao criar livro 3");

    // Busca os livros usando a nova função
    let ids = vec![
        book_id1.to_string(),
        book_id2.to_string(),
        book_id3.to_string(),
    ];
    
    let found_books = book_repository
        .find_by_ids(&ids)
        .await
        .expect("Falha ao buscar livros pelos IDs");

    // Verifica se todos os livros foram encontrados
    assert_eq!(
        found_books.len(),
        3,
        "Deveria encontrar os 3 livros criados"
    );

    // Verifica se os IDs correspondem aos esperados
    let found_ids: Vec<Uuid> = found_books.iter().map(|b| b.id).collect();
    assert!(found_ids.contains(&book_id1), "Deveria encontrar o livro 1");
    assert!(found_ids.contains(&book_id2), "Deveria encontrar o livro 2");
    assert!(found_ids.contains(&book_id3), "Deveria encontrar o livro 3");

    // Verifica os dados de um dos livros encontrados
    let found_book1 = found_books.iter().find(|b| b.id == book_id1).unwrap();
    assert_eq!(found_book1.book.google_id, book1.google_id);
    assert_eq!(found_book1.book.title, book1.title);
}

#[tokio::test]
async fn test_find_by_ids_empty_list() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;

    // Busca com lista vazia
    let ids: Vec<String> = vec![];
    
    let found_books = book_repository
        .find_by_ids(&ids)
        .await
        .expect("Falha ao buscar livros com lista vazia");

    // Verifica se o resultado é uma lista vazia
    assert!(
        found_books.is_empty(),
        "Deveria retornar uma lista vazia quando não há IDs"
    );
}

#[tokio::test]
async fn test_find_by_ids_invalid_ids() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;

    // Busca com IDs inválidos
    let ids = vec![
        "invalid-uuid".to_string(),
        "123".to_string(),
        "not-a-uuid".to_string(),
    ];
    
    let found_books = book_repository
        .find_by_ids(&ids)
        .await
        .expect("Falha ao buscar livros com IDs inválidos");

    // Verifica se o resultado é uma lista vazia
    assert!(
        found_books.is_empty(),
        "Deveria retornar uma lista vazia quando todos os IDs são inválidos"
    );
}

#[tokio::test]
async fn test_find_by_ids_mixed_valid_invalid() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let book_repository = setup_test_repository().await;

    // Cria um livro para o teste
    let book = create_test_book("test_find_mixed", true);

    // Insere o livro no banco de dados
    let book_id = book_repository
        .create(&book)
        .await
        .expect("Falha ao criar livro");

    // Lista com IDs válidos e inválidos
    let ids = vec![
        book_id.to_string(),
        "invalid-uuid".to_string(),
        "not-a-uuid".to_string(),
    ];
    
    let found_books = book_repository
        .find_by_ids(&ids)
        .await
        .expect("Falha ao buscar livros com IDs mistos");

    // Verifica se apenas o livro válido foi encontrado
    assert_eq!(
        found_books.len(),
        1,
        "Deveria encontrar apenas o livro com ID válido"
    );
    
    assert_eq!(found_books[0].id, book_id, "O ID encontrado deve corresponder ao ID válido");
} 