use crate::models::book::CreateBookOfferedDto;
use crate::repositories::book_repository::BookRepository;
use crate::repositories::books_offered_repository::BooksOfferedRepository;
use crate::repositories::books_offered_repository_test::{
    create_test_book, create_test_user, setup_book_repository, setup_test_repository, setup_user_repository,
};
use crate::repositories::test_helpers::get_test_mutex;
use crate::repositories::user_repository::UserRepository;
use uuid::Uuid;

#[tokio::test]
async fn test_find_books_offered() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Setup dos repositórios
    let book_repository = setup_book_repository().await;
    let user_repository = setup_user_repository().await;
    let books_offered_repository = setup_test_repository().await;

    // Cria um usuário para o teste
    let user = create_test_user();
    let user = user_repository.create(&user, "senha_hash".to_string()).await.unwrap();

    // Cria um livro para o teste
    let book = create_test_book("find_test123");
    let book_id = book_repository.create(&book).await.unwrap();

    // Cria um book_offered
    let book_offered = CreateBookOfferedDto {
        book_id,
        user_id: user.id,
    };

    books_offered_repository.create(&book_offered).await.unwrap();

    // Testa a busca do book_offered
    let result = books_offered_repository.find(&book_id, &user.id).await;
    
    assert!(result.is_ok(), "Falha ao buscar book_offered: {:?}", result.err());
    
    let found = result.unwrap();
    assert!(found.is_some(), "Book offered não encontrado");
    
    let found = found.unwrap();
    assert_eq!(found.book_id, book_id);
    assert_eq!(found.user_id, user.id);
}

#[tokio::test]
async fn test_find_books_offered_nonexistent_book() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Setup dos repositórios
    let user_repository = setup_user_repository().await;
    let books_offered_repository = setup_test_repository().await;

    // Cria um usuário para o teste
    let user = create_test_user();
    let user = user_repository.create(&user, "senha_hash".to_string()).await.unwrap();

    // Testa busca com book_id inexistente
    let non_existent_id = Uuid::new_v4();
    let result = books_offered_repository.find(&non_existent_id, &user.id).await;
    
    assert!(result.is_ok(), "A busca deveria ser bem-sucedida mesmo com ID inexistente");
    assert!(result.unwrap().is_none(), "Não deveria encontrar book_offered com ID de livro inexistente");
}

#[tokio::test]
async fn test_find_books_offered_nonexistent_user() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Setup dos repositórios
    let book_repository = setup_book_repository().await;
    let books_offered_repository = setup_test_repository().await;

    // Cria um livro para o teste
    let book = create_test_book("find_nonexistent_user_test");
    let book_id = book_repository.create(&book).await.unwrap();

    // Testa busca com user_id inexistente
    let non_existent_user_id = Uuid::new_v4();
    let result = books_offered_repository.find(&book_id, &non_existent_user_id).await;
    
    assert!(result.is_ok(), "A busca deveria ser bem-sucedida mesmo com user_id inexistente");
    assert!(result.unwrap().is_none(), "Não deveria encontrar book_offered com user_id inexistente");
}

#[tokio::test]
async fn test_find_books_offered_multiple_entries() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Setup dos repositórios
    let book_repository = setup_book_repository().await;
    let user_repository = setup_user_repository().await;
    let books_offered_repository = setup_test_repository().await;

    // Cria dois usuários para o teste
    let user1 = create_test_user();
    let user1 = user_repository.create(&user1, "senha_hash_1".to_string()).await.unwrap();
    
    let user2 = create_test_user();
    let user2 = user_repository.create(&user2, "senha_hash_2".to_string()).await.unwrap();

    // Cria dois livros para o teste
    let book1 = create_test_book("find_multi_test1");
    let book1_id = book_repository.create(&book1).await.unwrap();
    
    let book2 = create_test_book("find_multi_test2");
    let book2_id = book_repository.create(&book2).await.unwrap();

    // Cria várias relações de books_offered
    let book_offered1 = CreateBookOfferedDto {
        book_id: book1_id,
        user_id: user1.id,
    };
    
    let book_offered2 = CreateBookOfferedDto {
        book_id: book2_id,
        user_id: user1.id,
    };
    
    let book_offered3 = CreateBookOfferedDto {
        book_id: book1_id,
        user_id: user2.id,
    };

    // Insere as relações
    books_offered_repository.create(&book_offered1).await.unwrap();
    books_offered_repository.create(&book_offered2).await.unwrap();
    books_offered_repository.create(&book_offered3).await.unwrap();

    // Teste 1: Usuário 1 oferece o Livro 1
    let result1 = books_offered_repository.find(&book1_id, &user1.id).await.unwrap();
    assert!(result1.is_some(), "Usuário 1 deveria ter o Livro 1 como oferecido");
    
    // Teste 2: Usuário 1 oferece o Livro 2
    let result2 = books_offered_repository.find(&book2_id, &user1.id).await.unwrap();
    assert!(result2.is_some(), "Usuário 1 deveria ter o Livro 2 como oferecido");
    
    // Teste 3: Usuário 2 oferece o Livro 1
    let result3 = books_offered_repository.find(&book1_id, &user2.id).await.unwrap();
    assert!(result3.is_some(), "Usuário 2 deveria ter o Livro 1 como oferecido");
    
    // Teste 4: Usuário 2 NÃO oferece o Livro 2
    let result4 = books_offered_repository.find(&book2_id, &user2.id).await.unwrap();
    assert!(result4.is_none(), "Usuário 2 não deveria ter o Livro 2 como oferecido");
} 