use crate::{
    error::AppError,
    models::user::CreateUserDto,
    repositories::{
        test_helpers::{get_test_db_pool, get_test_mutex},
        user_repository::{PgUserRepository, UserRepository},
    },
};
use uuid::Uuid;

async fn setup_test_repository() -> PgUserRepository {
    // Obtém o pool de conexão com o banco de dados de teste
    let pool = get_test_db_pool().await;
    
    // Criamos o DatabasePool com o pool real
    PgUserRepository::new(pool)
}

#[tokio::test]
async fn test_create_user() {
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let user_repository = setup_test_repository().await;

    let user = CreateUserDto {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        password: "password".to_string(),
    };

    // Hash simulado para testes
    let hash_password = "hashed_password_for_test".to_string();

    let created_user = user_repository
        .create(&user, hash_password.clone())
        .await
        .expect("Falha ao criar usuário");

    assert_ne!(created_user.id, Uuid::nil());
    assert_eq!(created_user.name, "Test User");
    assert_eq!(created_user.email, "test@example.com");
    assert_eq!(created_user.hash_password, hash_password);
}

#[tokio::test]
async fn test_find_by_email() {
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let user_repository = setup_test_repository().await;

    // Cria um usuário para teste
    let user = CreateUserDto {
        name: "Email Test".to_string(),
        email: "find_by_email@example.com".to_string(),
        password: "password".to_string(),
    };
    let hash_password = "hashed_password_for_test".to_string();

    // Insere o usuário no banco
    let created_user = user_repository
        .create(&user, hash_password.clone())
        .await
        .expect("Falha ao criar usuário");

    // Testa a busca por email com email existente
    let found_user_opt = user_repository
        .find_by_email("find_by_email@example.com")
        .await
        .expect("Falha ao buscar usuário pelo email");

    assert!(found_user_opt.is_some());

    let found_user = found_user_opt.unwrap();
    assert_eq!(found_user.id, created_user.id);
    assert_eq!(found_user.email, "find_by_email@example.com");
    assert_eq!(found_user.name, "Email Test");

    // Testa a busca por email com email inexistente
    let non_existent_result = user_repository
        .find_by_email("nonexistent@example.com")
        .await
        .expect("Falha ao buscar usuário pelo email");

    assert!(non_existent_result.is_none());
}

#[tokio::test]
async fn test_email_exists() {
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let user_repository = setup_test_repository().await;

    // Cria um usuário para teste
    let user = CreateUserDto {
        name: "Email Exists Test".to_string(),
        email: "email_exists@example.com".to_string(),
        password: "password".to_string(),
    };
    let hash_password = "hashed_password_for_test".to_string();

    // Insere o usuário no banco
    user_repository.create(&user, hash_password.clone())
        .await
        .expect("Falha ao criar usuário");

    // Verifica se email existe através de find_by_email para testar a funcionalidade
    let found_user = user_repository
        .find_by_email("email_exists@example.com")
        .await
        .expect("Falha ao buscar usuário pelo email");

    assert!(found_user.is_some());

    // Verifica que email inexistente retorna None
    let not_found = user_repository
        .find_by_email("nonexistent@example.com")
        .await
        .expect("Falha ao buscar usuário pelo email");

    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_duplicate_email() {
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let user_repository = setup_test_repository().await;

    // Cria um primeiro usuário
    let user1 = CreateUserDto {
        name: "First User".to_string(),
        email: "duplicate@example.com".to_string(),
        password: "password".to_string(),
    };
    let hash_password = "hashed_password_for_test".to_string();

    // Insere o primeiro usuário
    user_repository.create(&user1, hash_password.clone())
        .await
        .expect("Falha ao criar primeiro usuário");

    // Tenta criar um segundo usuário com o mesmo email
    let user2 = CreateUserDto {
        name: "Second User".to_string(),
        email: "duplicate@example.com".to_string(),
        password: "different_password".to_string(),
    };

    // Deve falhar com erro de validação
    let result = user_repository.create(&user2, "another_hash".to_string()).await;
    assert!(result.is_err());

    // Verifica se é o tipo de erro esperado
    match result {
        Err(AppError::ValidationError(_)) => (),
        _ => panic!("Esperava erro de validação para email duplicado"),
    }
}
