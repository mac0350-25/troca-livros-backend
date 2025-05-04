use crate::models::book::CreateBookWantedDto;
use crate::repositories::book_repository::BookRepository;
use crate::repositories::books_wanted_repository::BooksWantedRepository;
use crate::repositories::books_wanted_repository_test::{
    create_test_book, create_test_user, setup_book_repository, setup_test_repository, setup_user_repository,
};
use crate::repositories::test_helpers::get_test_mutex;
use crate::repositories::user_repository::UserRepository;
use uuid::Uuid;

#[tokio::test]
async fn test_create_books_wanted() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Setup dos repositórios
    let book_repository = setup_book_repository().await;
    let user_repository = setup_user_repository().await;
    let books_wanted_repository = setup_test_repository().await;

    // Cria um usuário para o teste
    let user = create_test_user();
    let user = user_repository.create(&user, "senha_hash".to_string()).await.unwrap();

    // Cria um livro para o teste
    let book = create_test_book("test123");
    let book_id = book_repository.create(&book).await.unwrap();

    // Testa a criação de um book_wanted
    let book_wanted = CreateBookWantedDto {
        book_id,
        user_id: user.id,
    };

    let result = books_wanted_repository.create(&book_wanted).await;
    
    assert!(result.is_ok(), "Falha ao criar book_wanted: {:?}", result.err());
    
    let book_wanted = result.unwrap();
    assert_eq!(book_wanted.book_id, book_id);
    assert_eq!(book_wanted.user_id, user.id);
}

#[tokio::test]
async fn test_create_books_wanted_with_invalid_book_id() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Setup dos repositórios
    let user_repository = setup_user_repository().await;
    let books_wanted_repository = setup_test_repository().await;

    // Cria um usuário para o teste
    let user = create_test_user();
    let user = user_repository.create(&user, "senha_hash".to_string()).await.unwrap();

    // Tenta criar um book_wanted com um book_id inexistente
    let book_wanted = CreateBookWantedDto {
        book_id: Uuid::new_v4(),
        user_id: user.id,
    };

    let result = books_wanted_repository.create(&book_wanted).await;
    assert!(result.is_err(), "Deveria falhar ao criar com book_id inválido");
    
    let error = result.unwrap_err();
    assert!(
        format!("{:?}", error).contains("Livro com ID"), 
        "Erro deveria indicar que o livro não foi encontrado"
    );
}

#[tokio::test]
async fn test_create_books_wanted_with_invalid_user_id() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Setup dos repositórios
    let book_repository = setup_book_repository().await;
    let books_wanted_repository = setup_test_repository().await;

    // Cria um livro para o teste
    let book = create_test_book("invalid_user_test");
    let book_id = book_repository.create(&book).await.unwrap();

    // Tenta criar um book_wanted com um user_id inexistente
    let book_wanted = CreateBookWantedDto {
        book_id,
        user_id: Uuid::new_v4(),
    };

    let result = books_wanted_repository.create(&book_wanted).await;
    assert!(result.is_err(), "Deveria falhar ao criar com user_id inválido");
    
    let error = result.unwrap_err();
    assert!(
        format!("{:?}", error).contains("Usuário com ID"), 
        "Erro deveria indicar que o usuário não foi encontrado"
    );
}

#[tokio::test]
async fn test_create_books_wanted_duplicate() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Setup dos repositórios
    let book_repository = setup_book_repository().await;
    let user_repository = setup_user_repository().await;
    let books_wanted_repository = setup_test_repository().await;

    // Cria um usuário para o teste
    let user = create_test_user();
    let user = user_repository.create(&user, "senha_hash".to_string()).await.unwrap();

    // Cria um livro para o teste
    let book = create_test_book("duplicate_test");
    let book_id = book_repository.create(&book).await.unwrap();

    // Cria um book_wanted
    let book_wanted = CreateBookWantedDto {
        book_id,
        user_id: user.id,
    };

    // Primeira inserção deve ter sucesso
    let first_result = books_wanted_repository.create(&book_wanted).await;
    assert!(first_result.is_ok(), "A primeira inserção deveria ter sucesso");

    // Segunda inserção deve falhar com erro de duplicação
    let second_result = books_wanted_repository.create(&book_wanted).await;
    assert!(second_result.is_err(), "A segunda inserção deveria falhar");
    
    let error = second_result.unwrap_err();
    assert!(
        format!("{:?}", error).contains("já está na sua lista"), 
        "Erro deveria indicar que o livro já está na lista: {:?}", error
    );
} 