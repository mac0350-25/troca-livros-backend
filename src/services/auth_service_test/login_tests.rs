use super::*;
use crate::models::user::{LoginUserDto, User};
use crate::services::auth_service::AuthServiceImpl;
use mockall::predicate;
use std::sync::Arc;

/// Testa o login com credenciais válidas
///
/// Verifica se:
/// 1. O login é bem-sucedido com email e senha corretos
/// 2. O token é gerado corretamente
/// 3. Os dados do usuário são retornados corretamente
#[tokio::test]
async fn success_with_valid_credentials() {
    // Arrange
    let mut mock_repo = MockUserRepository::new();
    let user_id = Uuid::new_v4();
    let test_timestamp = create_test_timestamp();

    // Mock para retornar um usuário quando buscado por email
    mock_repo
        .expect_find_by_email()
        .with(predicate::eq("teste@example.com"))
        .returning(move |_| {
            Ok(Some(User {
                id: user_id,
                name: "Teste".to_string(),
                email: "teste@example.com".to_string(),
                hash_password: "hash_password".to_string(),
                created_at: test_timestamp,
                updated_at: test_timestamp,
            }))
        });

    // Criar mock do PasswordService configurado para retornar true (senha válida)
    let mock_password_service = create_mock_password_service("hash_dummy".to_string(), true);

    // Criar serviço de autenticação com os mocks
    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_repo),
        mock_password_service,
        create_test_config(),
    );

    // Act
    let login_dto = LoginUserDto {
        email: "teste@example.com".to_string(),
        password: "senha123".to_string(),
    };

    let result = auth_service.login(login_dto).await;

    // Assert
    assert!(result.is_ok(), "O login deveria ter sido bem-sucedido");

    if let Ok(token_response) = result {
        assert_eq!(token_response.token_type, "Bearer");
        assert!(!token_response.access_token.is_empty());
        assert_eq!(token_response.user.name, "Teste");
        assert_eq!(token_response.user.email, "teste@example.com");
    }
}

/// Testa falhas de validação em campos obrigatórios durante o login
///
/// Este teste parametrizado verifica se o login falha apropriadamente quando:
/// - O email está vazio
/// - O email tem formato inválido
/// - O email é muito longo
/// - A senha está vazia
///
/// E valida que a mensagem de erro contém o texto esperado em cada caso.
#[tokio::test]
async fn fail_with_invalid_fields() {
    // Casos de teste parametrizados
    let test_cases = vec![
        InvalidFieldTestCase {
            name: "email vazio",
            field_name: "email",
            user_name: "",  // não importa para login
            user_email: "", // Email vazio
            user_password: "senha123",
            expected_message: "O email não pode estar vazio",
        },
        InvalidFieldTestCase {
            name: "formato de email inválido",
            field_name: "email",
            user_name: "",                // não importa para login
            user_email: "email_invalido", // Email com formato inválido
            user_password: "senha123",
            expected_message: "Formato de email inválido",
        },
        // Para o teste de email longo, vamos adicioná-lo diretamente no teste
        // e não na lista de casos, para evitar problemas de lifetime
    ];

    // Executar testes para cada caso de validação
    for test_case in test_cases {
        // Arrange
        let mock_repo = MockUserRepository::new();

        // Criar mock do PasswordService (não será usado devido à validação falhar antes)
        let mock_password_service =
            create_mock_password_service("hashed_password".to_string(), true);

        let auth_service = AuthServiceImpl::new(
            Arc::new(mock_repo),
            mock_password_service,
            create_test_config(),
        );

        let login_dto = LoginUserDto {
            email: test_case.user_email.to_string(),
            password: test_case.user_password.to_string(),
        };

        // Act
        let result = auth_service.login(login_dto).await;

        // Assert
        assert!(
            result.is_err(),
            "O login deveria falhar com {} inválido/vazio",
            test_case.field_name
        );

        assert_validation_error_with_message(
            &result.unwrap_err(),
            test_case.expected_message,
            &format!("Erro ao validar {}", test_case.name),
        );
    }

    // Teste específico para email longo
    let mock_repo = MockUserRepository::new();
    let mock_password_service = create_mock_password_service("hashed_password".to_string(), true);
    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_repo),
        mock_password_service,
        create_test_config(),
    );

    // Criar um email longo (acima do limite)
    let long_email = "A".repeat(256);
    let login_dto = LoginUserDto {
        email: long_email,
        password: "senha123".to_string(),
    };

    // Act
    let result = auth_service.login(login_dto).await;

    // Assert
    assert!(
        result.is_err(),
        "O login deveria falhar com email muito longo"
    );
    assert_validation_error_with_message(
        &result.unwrap_err(),
        "O email deve ter menos de 255 caracteres",
        "Erro ao validar email muito longo",
    );

    // Teste para senha vazia
    let mock_repo = MockUserRepository::new();
    let mock_password_service = create_mock_password_service("hashed_password".to_string(), true);
    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_repo),
        mock_password_service,
        create_test_config(),
    );

    let login_dto = LoginUserDto {
        email: "teste@example.com".to_string(),
        password: "".to_string(),
    };

    // Act
    let result = auth_service.login(login_dto).await;

    // Assert
    assert!(result.is_err(), "O login deveria falhar com senha vazia");
    assert_validation_error_with_message(
        &result.unwrap_err(),
        "A senha não pode estar vazia",
        "Erro ao validar senha vazia",
    );
}

/// Testa o login com email inexistente
///
/// Verifica se:
/// 1. O login falha quando o email não existe no banco
/// 2. A mensagem de erro indica "Credenciais inválidas" (sem expor se o email existe ou não)
#[tokio::test]
async fn fail_with_invalid_email() {
    // Arrange
    let mut mock_repo = MockUserRepository::new();

    // Mock retorna None (usuário não encontrado)
    mock_repo
        .expect_find_by_email()
        .with(predicate::eq("nao_existe@example.com"))
        .returning(|_| Ok(None));

    // Criar mock do PasswordService (não importa a configuração, pois o email não existe)
    let mock_password_service = create_mock_password_service("hash_dummy".to_string(), true);

    // Criar serviço de autenticação com os mocks
    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_repo),
        mock_password_service,
        create_test_config(),
    );

    // Act
    let login_dto = LoginUserDto {
        email: "nao_existe@example.com".to_string(),
        password: "senha123".to_string(),
    };

    let result = auth_service.login(login_dto).await;

    // Assert
    assert!(result.is_err(), "O login deveria falhar com email inválido");
    assert_auth_error_with_message(
        &result.unwrap_err(),
        "Credenciais inválidas",
        "Erro ao fazer login com email inválido",
    );
}

/// Testa o login com senha incorreta
///
/// Verifica se:
/// 1. O login falha quando a senha está incorreta
/// 2. A mensagem de erro indica "Credenciais inválidas" (sem especificar que a senha está errada)
#[tokio::test]
async fn fail_with_invalid_password() {
    // Arrange
    let mut mock_repo = MockUserRepository::new();
    let user_id = Uuid::new_v4();
    let test_timestamp = create_test_timestamp();

    // Mock retorna um usuário (email existe)
    mock_repo
        .expect_find_by_email()
        .with(predicate::eq("teste@example.com"))
        .returning(move |_| {
            Ok(Some(User {
                id: user_id,
                name: "Teste".to_string(),
                email: "teste@example.com".to_string(),
                hash_password: "hash_password".to_string(),
                created_at: test_timestamp,
                updated_at: test_timestamp,
            }))
        });

    // Criar mock do PasswordService configurado para retornar false (senha inválida)
    let mock_password_service = create_mock_password_service("hash_dummy".to_string(), false);

    // Criar serviço de autenticação com os mocks
    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_repo),
        mock_password_service,
        create_test_config(),
    );

    // Act
    let login_dto = LoginUserDto {
        email: "teste@example.com".to_string(),
        password: "senha_incorreta".to_string(),
    };

    let result = auth_service.login(login_dto).await;

    // Assert
    assert!(
        result.is_err(),
        "O login deveria falhar com senha incorreta"
    );
    assert_auth_error_with_message(
        &result.unwrap_err(),
        "Credenciais inválidas",
        "Erro ao fazer login com senha incorreta",
    );
}
