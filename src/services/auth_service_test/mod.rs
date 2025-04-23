use async_trait::async_trait;
use chrono::DateTime;
use mockall::mock;
use std::sync::Arc;
use uuid::Uuid;

use crate::config::Config;
use crate::error::AppError;
use crate::models::user::{CreateUserDto, User};
use crate::repositories::user_repository::UserRepository;
use crate::services::auth_service::AuthService;
use crate::services::password_service::PasswordService;

// Módulos de testes
pub mod login_tests;
pub mod register_tests;

// ----- Mocks para os testes -----

// Mock do UserRepository
mock! {
    pub UserRepository {}

    #[async_trait]
    impl UserRepository for UserRepository {
        async fn create(&self, user: &CreateUserDto, hash_password: String) -> Result<User, AppError>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    }
}

// Mock do PasswordService
mock! {
    pub PasswordService {}

    #[async_trait]
    impl PasswordService for PasswordService {
        fn hash_password(&self, password: &str) -> Result<String, AppError>;
        fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError>;
    }
}

// ----- Funções auxiliares para preparação de testes -----

/// Cria uma configuração padrão para testes
pub fn create_test_config() -> Config {
    Config {
        database_url: "postgres://dummy".to_string(),
        jwt_secret: "test_secret".to_string(),
        jwt_expires_in: "1".to_string(),
        port: 8080,
    }
}

/// Cria um timestamp fictício para testes
pub fn create_test_timestamp() -> chrono::NaiveDateTime {
    let timestamp = DateTime::from_timestamp(61, 0).unwrap();
    timestamp.naive_utc()
}

/// Cria um usuário fictício para testes
pub fn create_test_user(name: &str, email: &str) -> User {
    User {
        id: Uuid::new_v4(),
        name: name.to_string(),
        email: email.to_string(),
        hash_password: "hashed_password".to_string(),
        created_at: create_test_timestamp(),
        updated_at: create_test_timestamp(),
    }
}

/// Cria um DTO de usuário para testes
pub fn create_user_dto(name: &str, email: &str, password: &str) -> CreateUserDto {
    CreateUserDto {
        name: name.to_string(),
        email: email.to_string(),
        password: password.to_string(),
    }
}

/// Cria um mock do PasswordService configurado para testes
pub fn create_mock_password_service(
    hash_result: String,
    verify_result: bool,
) -> Arc<MockPasswordService> {
    let mut mock_service = MockPasswordService::new();

    // Configure o comportamento do mock para hash_password
    mock_service
        .expect_hash_password()
        .returning(move |_| Ok(hash_result.clone()));

    // Configure o comportamento do mock para verify_password
    mock_service
        .expect_verify_password()
        .returning(move |_, _| Ok(verify_result));

    Arc::new(mock_service)
}

// Struct para facilitar parametrização de testes
pub struct InvalidFieldTestCase {
    pub name: &'static str,             // Nome do caso de teste
    pub field_name: &'static str,       // Nome do campo sendo testado
    pub user_name: &'static str,        // Valor para o campo nome
    pub user_email: &'static str,       // Valor para o campo email
    pub user_password: &'static str,    // Valor para o campo senha
    pub expected_message: &'static str, // Mensagem de erro esperada
}

// ----- Funções auxiliares para verificação de erros -----

/// Verifica se o erro é do tipo esperado e contém a mensagem esperada
pub fn assert_error_with_message(
    err: &AppError,
    error_type: &str,
    expected_message: &str,
    context: &str,
) {
    match (err, error_type) {
        (AppError::ValidationError(msg), "ValidationError") => {
            assert!(
                msg.contains(expected_message),
                "{}: Mensagem de erro inesperada: '{}', esperava que contivesse: '{}'",
                context,
                msg,
                expected_message
            );
        }
        (AppError::AuthError(msg), "AuthError") => {
            assert!(
                msg.contains(expected_message),
                "{}: Mensagem de erro inesperada: '{}', esperava que contivesse: '{}'",
                context,
                msg,
                expected_message
            );
        }
        (AppError::DatabaseError(msg), "DatabaseError") => {
            assert!(
                msg.contains(expected_message),
                "{}: Mensagem de erro inesperada: '{}', esperava que contivesse: '{}'",
                context,
                msg,
                expected_message
            );
        }
        _ => panic!(
            "{}: Tipo de erro inesperado: {:?}, esperava {}",
            context, err, error_type
        ),
    }
}

/// Verifica se o erro é do tipo ValidationError e contém a mensagem esperada
pub fn assert_validation_error_with_message(err: &AppError, expected_message: &str, context: &str) {
    assert_error_with_message(err, "ValidationError", expected_message, context);
}

/// Verifica se o erro é do tipo AuthError e contém a mensagem esperada
pub fn assert_auth_error_with_message(err: &AppError, expected_message: &str, context: &str) {
    assert_error_with_message(err, "AuthError", expected_message, context);
}
