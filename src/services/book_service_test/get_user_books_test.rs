use std::sync::Arc;
use uuid::Uuid;

use crate::error::AppError;
use crate::services::book_service::{BookService, BookServiceImpl};
use crate::services::test_mocks::{
    create_test_book_with_id, MockBookRepository, MockBooksOfferedRepository, MockBooksWantedRepository,
};

#[tokio::test]
async fn test_get_user_books_success() {
    // Arrange - Configurar os mocks e dados de teste
    let user_id = Uuid::new_v4();
    
    // Criar IDs para os livros possuídos e desejados
    let offered_book_id1 = Uuid::new_v4();
    let offered_book_id2 = Uuid::new_v4();
    let wanted_book_id1 = Uuid::new_v4();
    let wanted_book_id2 = Uuid::new_v4();
    
    // Configurar o mock do BooksOfferedRepository
    let mut mock_books_offered_repo = MockBooksOfferedRepository::new();
    mock_books_offered_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(move |_| Ok(vec![offered_book_id1, offered_book_id2]));
    
    // Configurar o mock do BooksWantedRepository
    let mut mock_books_wanted_repo = MockBooksWantedRepository::new();
    mock_books_wanted_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(move |_| Ok(vec![wanted_book_id1, wanted_book_id2]));
    
    // Configurar o mock do BookRepository para os livros possuídos
    let mut mock_book_repo = MockBookRepository::new();
    
    // Mock para livros possuídos
    let offered_book1 = create_test_book_with_id(offered_book_id1, "offered_google_id_1");
    let offered_book2 = create_test_book_with_id(offered_book_id2, "offered_google_id_2");
    let offered_books = vec![offered_book1.clone(), offered_book2.clone()];
    
    // Mock para livros desejados
    let wanted_book1 = create_test_book_with_id(wanted_book_id1, "wanted_google_id_1");
    let wanted_book2 = create_test_book_with_id(wanted_book_id2, "wanted_google_id_2");
    let wanted_books = vec![wanted_book1.clone(), wanted_book2.clone()];
    
    // Configurar expectativa para find_by_ids com livros possuídos
    mock_book_repo
        .expect_find_by_ids()
        .with(mockall::predicate::function(move |ids: &[String]| {
            ids.contains(&offered_book_id1.to_string()) && ids.contains(&offered_book_id2.to_string())
        }))
        .times(1)
        .returning(move |_| Ok(offered_books.clone()));
    
    // Configurar expectativa para find_by_ids com livros desejados
    mock_book_repo
        .expect_find_by_ids()
        .with(mockall::predicate::function(move |ids: &[String]| {
            ids.contains(&wanted_book_id1.to_string()) && ids.contains(&wanted_book_id2.to_string())
        }))
        .times(1)
        .returning(move |_| Ok(wanted_books.clone()));
    
    // Criar o serviço com os mocks
    let book_service = BookServiceImpl::new(
        Arc::new(mock_book_repo),
        Arc::new(mock_books_offered_repo),
        Arc::new(mock_books_wanted_repo),
    );
    
    // Act - Chamar a função a ser testada
    let result = book_service.get_user_books(&user_id).await;
    
    // Assert - Verificar os resultados
    assert!(result.is_ok(), "A função deveria retornar Ok");
    
    let user_books = result.unwrap();
    
    // Verificar os livros possuídos
    assert_eq!(user_books.offered_books.len(), 2, "Deveria ter 2 livros possuídos");
    assert!(
        user_books.offered_books.iter().any(|b| b.id == offered_book_id1),
        "Livro possuído 1 não encontrado"
    );
    assert!(
        user_books.offered_books.iter().any(|b| b.id == offered_book_id2),
        "Livro possuído 2 não encontrado"
    );
    
    // Verificar os livros desejados
    assert_eq!(user_books.wanted_books.len(), 2, "Deveria ter 2 livros desejados");
    assert!(
        user_books.wanted_books.iter().any(|b| b.id == wanted_book_id1),
        "Livro desejado 1 não encontrado"
    );
    assert!(
        user_books.wanted_books.iter().any(|b| b.id == wanted_book_id2),
        "Livro desejado 2 não encontrado"
    );
}

#[tokio::test]
async fn test_get_user_books_empty() {
    // Arrange - Configurar os mocks para um usuário sem livros
    let user_id = Uuid::new_v4();
    
    // Configurar o mock do BooksOfferedRepository para retornar lista vazia
    let mut mock_books_offered_repo = MockBooksOfferedRepository::new();
    mock_books_offered_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Ok(vec![]));
    
    // Configurar o mock do BooksWantedRepository para retornar lista vazia
    let mut mock_books_wanted_repo = MockBooksWantedRepository::new();
    mock_books_wanted_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Ok(vec![]));
    
    // Configurar o mock do BookRepository
    let mock_book_repo = MockBookRepository::new();
    // Não precisamos configurar expectativas para find_by_ids porque
    // não deve ser chamado quando as listas estão vazias
    
    // Criar o serviço com os mocks
    let book_service = BookServiceImpl::new(
        Arc::new(mock_book_repo),
        Arc::new(mock_books_offered_repo),
        Arc::new(mock_books_wanted_repo),
    );
    
    // Act - Chamar a função a ser testada
    let result = book_service.get_user_books(&user_id).await;
    
    // Assert - Verificar os resultados
    assert!(result.is_ok(), "A função deveria retornar Ok");
    
    let user_books = result.unwrap();
    
    // Verificar que ambas as listas estão vazias
    assert!(user_books.offered_books.is_empty(), "A lista de livros possuídos deveria estar vazia");
    assert!(user_books.wanted_books.is_empty(), "A lista de livros desejados deveria estar vazia");
}

#[tokio::test]
async fn test_get_user_books_only_offered() {
    // Arrange - Configurar os mocks para um usuário com apenas livros possuídos
    let user_id = Uuid::new_v4();
    let offered_book_id = Uuid::new_v4();
    
    // Configurar o mock do BooksOfferedRepository
    let mut mock_books_offered_repo = MockBooksOfferedRepository::new();
    mock_books_offered_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(move |_| Ok(vec![offered_book_id]));
    
    // Configurar o mock do BooksWantedRepository para retornar lista vazia
    let mut mock_books_wanted_repo = MockBooksWantedRepository::new();
    mock_books_wanted_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Ok(vec![]));
    
    // Configurar o mock do BookRepository
    let mut mock_book_repo = MockBookRepository::new();
    
    // Mock para o livro possuído
    let offered_book = create_test_book_with_id(offered_book_id, "offered_google_id");
    
    // Configurar expectativa para find_by_ids com livro possuído
    mock_book_repo
        .expect_find_by_ids()
        .with(mockall::predicate::function(move |ids: &[String]| {
            ids.len() == 1 && ids[0] == offered_book_id.to_string()
        }))
        .times(1)
        .returning(move |_| Ok(vec![offered_book.clone()]));
    
    // Criar o serviço com os mocks
    let book_service = BookServiceImpl::new(
        Arc::new(mock_book_repo),
        Arc::new(mock_books_offered_repo),
        Arc::new(mock_books_wanted_repo),
    );
    
    // Act - Chamar a função a ser testada
    let result = book_service.get_user_books(&user_id).await;
    
    // Assert - Verificar os resultados
    assert!(result.is_ok(), "A função deveria retornar Ok");
    
    let user_books = result.unwrap();
    
    // Verificar que há apenas livros possuídos
    assert_eq!(user_books.offered_books.len(), 1, "Deveria ter 1 livro possuído");
    assert_eq!(user_books.offered_books[0].id, offered_book_id, "ID do livro possuído incorreto");
    assert!(user_books.wanted_books.is_empty(), "A lista de livros desejados deveria estar vazia");
}

#[tokio::test]
async fn test_get_user_books_only_wanted() {
    // Arrange - Configurar os mocks para um usuário com apenas livros desejados
    let user_id = Uuid::new_v4();
    let wanted_book_id = Uuid::new_v4();
    
    // Configurar o mock do BooksOfferedRepository para retornar lista vazia
    let mut mock_books_offered_repo = MockBooksOfferedRepository::new();
    mock_books_offered_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Ok(vec![]));
    
    // Configurar o mock do BooksWantedRepository
    let mut mock_books_wanted_repo = MockBooksWantedRepository::new();
    mock_books_wanted_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(move |_| Ok(vec![wanted_book_id]));
    
    // Configurar o mock do BookRepository
    let mut mock_book_repo = MockBookRepository::new();
    
    // Mock para o livro desejado
    let wanted_book = create_test_book_with_id(wanted_book_id, "wanted_google_id");
    
    // Configurar expectativa para find_by_ids com livro desejado
    mock_book_repo
        .expect_find_by_ids()
        .with(mockall::predicate::function(move |ids: &[String]| {
            ids.len() == 1 && ids[0] == wanted_book_id.to_string()
        }))
        .times(1)
        .returning(move |_| Ok(vec![wanted_book.clone()]));
    
    // Criar o serviço com os mocks
    let book_service = BookServiceImpl::new(
        Arc::new(mock_book_repo),
        Arc::new(mock_books_offered_repo),
        Arc::new(mock_books_wanted_repo),
    );
    
    // Act - Chamar a função a ser testada
    let result = book_service.get_user_books(&user_id).await;
    
    // Assert - Verificar os resultados
    assert!(result.is_ok(), "A função deveria retornar Ok");
    
    let user_books = result.unwrap();
    
    // Verificar que há apenas livros desejados
    assert!(user_books.offered_books.is_empty(), "A lista de livros possuídos deveria estar vazia");
    assert_eq!(user_books.wanted_books.len(), 1, "Deveria ter 1 livro desejado");
    assert_eq!(user_books.wanted_books[0].id, wanted_book_id, "ID do livro desejado incorreto");
}

#[tokio::test]
async fn test_get_user_books_error_in_offered_repository() {
    // Arrange - Configurar os mocks com erro no BooksOfferedRepository
    let user_id = Uuid::new_v4();
    
    // Configurar o mock do BooksOfferedRepository para retornar erro
    let mut mock_books_offered_repo = MockBooksOfferedRepository::new();
    mock_books_offered_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Err(AppError::DatabaseError("Erro de banco de dados simulado".to_string())));
    
    // Configurar mocks que não devem ser chamados
    let mock_books_wanted_repo = MockBooksWantedRepository::new();
    let mock_book_repo = MockBookRepository::new();
    
    // Criar o serviço com os mocks
    let book_service = BookServiceImpl::new(
        Arc::new(mock_book_repo),
        Arc::new(mock_books_offered_repo),
        Arc::new(mock_books_wanted_repo),
    );
    
    // Act - Chamar a função a ser testada
    let result = book_service.get_user_books(&user_id).await;
    
    // Assert - Verificar que houve erro
    assert!(result.is_err(), "A função deveria retornar um erro");
    
    match result {
        Err(AppError::DatabaseError(_)) => (), // Erro esperado
        Err(e) => panic!("Erro de tipo inesperado: {:?}", e),
        Ok(_) => panic!("Deveria ter retornado um erro"),
    }
}

#[tokio::test]
async fn test_get_user_books_error_in_wanted_repository() {
    // Arrange - Configurar os mocks com erro no BooksWantedRepository
    let user_id = Uuid::new_v4();
    
    // Configurar o mock do BooksOfferedRepository
    let mut mock_books_offered_repo = MockBooksOfferedRepository::new();
    mock_books_offered_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Ok(vec![]));
    
    // Configurar o mock do BooksWantedRepository para retornar erro
    let mut mock_books_wanted_repo = MockBooksWantedRepository::new();
    mock_books_wanted_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Err(AppError::DatabaseError("Erro de banco de dados simulado".to_string())));
    
    // Configurar mock que não deve ser chamado
    let mock_book_repo = MockBookRepository::new();
    
    // Criar o serviço com os mocks
    let book_service = BookServiceImpl::new(
        Arc::new(mock_book_repo),
        Arc::new(mock_books_offered_repo),
        Arc::new(mock_books_wanted_repo),
    );
    
    // Act - Chamar a função a ser testada
    let result = book_service.get_user_books(&user_id).await;
    
    // Assert - Verificar que houve erro
    assert!(result.is_err(), "A função deveria retornar um erro");
    
    match result {
        Err(AppError::DatabaseError(_)) => (), // Erro esperado
        Err(e) => panic!("Erro de tipo inesperado: {:?}", e),
        Ok(_) => panic!("Deveria ter retornado um erro"),
    }
}

#[tokio::test]
async fn test_get_user_books_error_in_book_repository() {
    // Arrange - Configurar os mocks com erro no BookRepository
    let user_id = Uuid::new_v4();
    let book_id = Uuid::new_v4();
    
    // Configurar o mock do BooksOfferedRepository
    let mut mock_books_offered_repo = MockBooksOfferedRepository::new();
    mock_books_offered_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(move |_| Ok(vec![book_id]));
    
    // Configurar o mock do BooksWantedRepository
    let mut mock_books_wanted_repo = MockBooksWantedRepository::new();
    mock_books_wanted_repo
        .expect_find_by_user_id()
        .with(mockall::predicate::eq(user_id))
        .times(1)
        .returning(|_| Ok(vec![]));
    
    // Configurar o mock do BookRepository para retornar erro
    let mut mock_book_repo = MockBookRepository::new();
    mock_book_repo
        .expect_find_by_ids()
        .times(1)
        .returning(|_| Err(AppError::DatabaseError("Erro de banco de dados simulado".to_string())));
    
    // Criar o serviço com os mocks
    let book_service = BookServiceImpl::new(
        Arc::new(mock_book_repo),
        Arc::new(mock_books_offered_repo),
        Arc::new(mock_books_wanted_repo),
    );
    
    // Act - Chamar a função a ser testada
    let result = book_service.get_user_books(&user_id).await;
    
    // Assert - Verificar que houve erro
    assert!(result.is_err(), "A função deveria retornar um erro");
    
    match result {
        Err(AppError::DatabaseError(_)) => (), // Erro esperado
        Err(e) => panic!("Erro de tipo inesperado: {:?}", e),
        Ok(_) => panic!("Deveria ter retornado um erro"),
    }
} 