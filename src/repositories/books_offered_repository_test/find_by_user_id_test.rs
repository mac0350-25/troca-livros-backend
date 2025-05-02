use std::sync::Arc;
use uuid::Uuid;

use crate::models::book::CreateBookOfferedDto;
use crate::repositories::book_repository::BookRepository;
use crate::repositories::books_offered_repository::BooksOfferedRepository;
use crate::repositories::user_repository::UserRepository;
use crate::repositories::books_offered_repository_test::{
    create_test_book, create_test_user, setup_book_repository, setup_test_repository,
    setup_user_repository,
};
use crate::repositories::test_helpers::get_test_mutex;

#[sqlx::test]
async fn should_find_books_offered_by_user_id() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;
    // Arrange
    let user_repository = setup_user_repository().await;
    let book_repository = setup_book_repository().await;
    let books_offered_repository = Arc::new(setup_test_repository().await);

    // Criar um usuário para teste
    let test_user = create_test_user();
    let user = user_repository.create(&test_user, "hashed_password".to_string()).await.unwrap();

    // Criar alguns livros para teste
    let test_book1 = create_test_book("test_id_1");
    let book_id1 = book_repository.create(&test_book1).await.unwrap();

    let test_book2 = create_test_book("test_id_2");
    let book_id2 = book_repository.create(&test_book2).await.unwrap();

    let test_book3 = create_test_book("test_id_3");
    let book_id3 = book_repository.create(&test_book3).await.unwrap();

    // Adicionar livros à lista de oferecidos do usuário
    let book_offered1 = CreateBookOfferedDto {
        book_id: book_id1,
        user_id: user.id,
    };
    books_offered_repository.create(&book_offered1).await.unwrap();

    let book_offered2 = CreateBookOfferedDto {
        book_id: book_id2,
        user_id: user.id,
    };
    books_offered_repository.create(&book_offered2).await.unwrap();

    // Act
    let result = books_offered_repository.find_by_user_id(&user.id).await.unwrap();

    // Assert
    assert_eq!(result.len(), 2);
    assert!(result.contains(&book_id1));
    assert!(result.contains(&book_id2));
    assert!(!result.contains(&book_id3));
}

#[sqlx::test]
async fn should_return_empty_vector_when_user_has_no_books() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Arrange
    let user_repository = setup_user_repository().await;
    let books_offered_repository = Arc::new(setup_test_repository().await);

    // Criar um usuário para teste
    let test_user = create_test_user();
    let user = user_repository.create(&test_user, "hashed_password".to_string()).await.unwrap();

    // Act
    let result = books_offered_repository.find_by_user_id(&user.id).await.unwrap();

    // Assert
    assert_eq!(result.len(), 0);
}

#[sqlx::test]
async fn should_return_only_books_from_specific_user() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Arrange
    let user_repository = setup_user_repository().await;
    let book_repository = setup_book_repository().await;
    let books_offered_repository = Arc::new(setup_test_repository().await);

    // Criar dois usuários para teste
    let test_user1 = create_test_user();
    let user1 = user_repository.create(&test_user1, "hashed_password".to_string()).await.unwrap();

    let test_user2 = create_test_user();
    let user2 = user_repository.create(&test_user2, "hashed_password".to_string()).await.unwrap();

    // Criar alguns livros para teste
    let test_book1 = create_test_book("test_id_1");
    let book_id1 = book_repository.create(&test_book1).await.unwrap();

    let test_book2 = create_test_book("test_id_2");
    let book_id2 = book_repository.create(&test_book2).await.unwrap();

    // Adicionar livro à lista do primeiro usuário
    let book_offered1 = CreateBookOfferedDto {
        book_id: book_id1,
        user_id: user1.id,
    };
    books_offered_repository.create(&book_offered1).await.unwrap();

    // Adicionar livro à lista do segundo usuário
    let book_offered2 = CreateBookOfferedDto {
        book_id: book_id2,
        user_id: user2.id,
    };
    books_offered_repository.create(&book_offered2).await.unwrap();

    // Act
    let result1 = books_offered_repository.find_by_user_id(&user1.id).await.unwrap();
    let result2 = books_offered_repository.find_by_user_id(&user2.id).await.unwrap();

    // Assert
    assert_eq!(result1.len(), 1);
    assert!(result1.contains(&book_id1));
    assert!(!result1.contains(&book_id2));

    assert_eq!(result2.len(), 1);
    assert!(result2.contains(&book_id2));
    assert!(!result2.contains(&book_id1));
}

#[sqlx::test]
async fn should_return_empty_vector_for_non_existent_user() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Arrange
    let books_offered_repository = Arc::new(setup_test_repository().await);
    let non_existent_user_id = Uuid::new_v4();

    // Act
    let result = books_offered_repository.find_by_user_id(&non_existent_user_id).await.unwrap();

    // Assert
    assert_eq!(result.len(), 0);
}