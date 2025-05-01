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
async fn test_delete_books_wanted() {
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
    let book = create_test_book("delete_test123");
    let book_id = book_repository.create(&book).await.unwrap();

    // Cria um book_wanted
    let book_wanted = CreateBookWantedDto {
        book_id,
        user_id: user.id,
    };

    books_wanted_repository.create(&book_wanted).await.unwrap();

    // Verifica se o book_wanted foi criado corretamente
    let find_result_before = books_wanted_repository.find(&book_id, &user.id).await.unwrap();
    assert!(find_result_before.is_some(), "O book_wanted deveria existir antes da deleção");

    // Testa a deleção do book_wanted
    let result = books_wanted_repository.delete(&book_id, &user.id).await;
    
    assert!(result.is_ok(), "Falha ao deletar book_wanted: {:?}", result.err());
    assert!(result.unwrap(), "A deleção deveria retornar true");

    // Verifica se foi realmente deletado
    let find_result_after = books_wanted_repository.find(&book_id, &user.id).await.unwrap();
    assert!(find_result_after.is_none(), "O book_wanted não foi deletado corretamente");
}

#[tokio::test]
async fn test_delete_books_wanted_nonexistent() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Setup dos repositórios
    let user_repository = setup_user_repository().await;
    let books_wanted_repository = setup_test_repository().await;

    // Cria um usuário para o teste
    let user = create_test_user();
    let user = user_repository.create(&user, "senha_hash".to_string()).await.unwrap();

    // Tenta deletar um book_wanted inexistente
    let non_existent_id = Uuid::new_v4();
    let result = books_wanted_repository.delete(&non_existent_id, &user.id).await;
    
    assert!(result.is_ok(), "A deleção deveria ser bem-sucedida mesmo com ID inexistente");
    assert!(!result.unwrap(), "Deleção de ID inexistente deveria retornar false");
}

#[tokio::test]
async fn test_delete_books_wanted_multiple_users() {
    // Usa mutex para garantir execução sequencial dos testes
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    // Setup dos repositórios
    let book_repository = setup_book_repository().await;
    let user_repository = setup_user_repository().await;
    let books_wanted_repository = setup_test_repository().await;

    // Cria dois usuários para o teste
    let user1 = create_test_user();
    let user1 = user_repository.create(&user1, "senha_hash_1".to_string()).await.unwrap();
    
    let user2 = create_test_user();
    let user2 = user_repository.create(&user2, "senha_hash_2".to_string()).await.unwrap();

    // Cria um livro para o teste
    let book = create_test_book("delete_multi_test");
    let book_id = book_repository.create(&book).await.unwrap();

    // Cria books_wanted para ambos os usuários
    let book_wanted1 = CreateBookWantedDto {
        book_id,
        user_id: user1.id,
    };
    
    let book_wanted2 = CreateBookWantedDto {
        book_id,
        user_id: user2.id,
    };

    // Insere as relações
    books_wanted_repository.create(&book_wanted1).await.unwrap();
    books_wanted_repository.create(&book_wanted2).await.unwrap();

    // Deleta apenas para o usuário 1
    let delete_result = books_wanted_repository.delete(&book_id, &user1.id).await.unwrap();
    assert!(delete_result, "A deleção para o usuário 1 deveria ter sucesso");

    // Verifica se foi removido para o usuário 1
    let find_result1 = books_wanted_repository.find(&book_id, &user1.id).await.unwrap();
    assert!(find_result1.is_none(), "O livro não deveria mais estar oferecido pelo usuário 1");

    // Verifica se ainda existe para o usuário 2
    let find_result2 = books_wanted_repository.find(&book_id, &user2.id).await.unwrap();
    assert!(find_result2.is_some(), "O livro ainda deveria estar oferecido pelo usuário 2");
}

#[tokio::test]
async fn test_delete_books_wanted_multiple_books() {
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

    // Cria dois livros para o teste
    let book1 = create_test_book("delete_book1_test");
    let book1_id = book_repository.create(&book1).await.unwrap();
    
    let book2 = create_test_book("delete_book2_test");
    let book2_id = book_repository.create(&book2).await.unwrap();

    // Cria books_wanted para ambos os livros
    let book_wanted1 = CreateBookWantedDto {
        book_id: book1_id,
        user_id: user.id,
    };
    
    let book_wanted2 = CreateBookWantedDto {
        book_id: book2_id,
        user_id: user.id,
    };

    // Insere as relações
    books_wanted_repository.create(&book_wanted1).await.unwrap();
    books_wanted_repository.create(&book_wanted2).await.unwrap();

    // Deleta apenas o livro 1
    let delete_result = books_wanted_repository.delete(&book1_id, &user.id).await.unwrap();
    assert!(delete_result, "A deleção para o livro 1 deveria ter sucesso");

    // Verifica se o livro 1 foi removido
    let find_result1 = books_wanted_repository.find(&book1_id, &user.id).await.unwrap();
    assert!(find_result1.is_none(), "O livro 1 não deveria mais estar oferecido");

    // Verifica se o livro 2 ainda existe
    let find_result2 = books_wanted_repository.find(&book2_id, &user.id).await.unwrap();
    assert!(find_result2.is_some(), "O livro 2 ainda deveria estar oferecido");
} 