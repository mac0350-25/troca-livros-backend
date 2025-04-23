use super::*;
use crate::error::AppError;
use crate::services::auth_service::AuthServiceImpl;
use mockall::predicate;
use std::sync::Arc;

/// Testa o registro de usuário com dados válidos
///
/// Verifica se:
/// 1. O registro é bem-sucedido quando todos os campos são válidos
/// 2. O objeto UserResponse retornado contém os dados corretos do usuário
#[tokio::test]
async fn success_with_valid_data() {
    // Arrange
    let mut mock_repo = MockUserRepository::new();

    // Configurar mock para retornar sucesso
    mock_repo
        .expect_create()
        .with(
            predicate::function(|user: &CreateUserDto| {
                user.name == "Teste"
                    && user.email == "teste@example.com"
                    && user.password.len() >= 6
            }),
            predicate::always(),
        )
        .returning(|user, _| Ok(create_test_user(&user.name, &user.email)));

    // Criar mock do PasswordService que retorna um hash fixo
    let mock_password_service = create_mock_password_service("hashed_password".to_string(), true);

    // Criar o serviço com os mocks
    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_repo),
        mock_password_service,
        create_test_config(),
    );

    let dto = create_user_dto("Teste", "teste@example.com", "senha123");

    // Act
    let result = auth_service.register(dto).await;

    // Assert
    assert!(result.is_ok(), "O registro deveria ter sido bem-sucedido");

    if let Ok(user_response) = result {
        assert_eq!(user_response.name, "Teste");
        assert_eq!(user_response.email, "teste@example.com");
    }
}

/// Testa falhas de validação em campos obrigatórios durante o registro
///
/// Este teste parametrizado verifica se o registro falha apropriadamente quando:
/// - O nome está vazio
/// - O email está vazio
/// - A senha está vazia
///
/// E valida que a mensagem de erro contém o texto esperado em cada caso.
#[tokio::test]
async fn fail_with_invalid_fields() {
    // Casos de teste parametrizados
    let test_cases = vec![
        InvalidFieldTestCase {
            name: "nome vazio",
            field_name: "nome",
            user_name: "",
            user_email: "teste@example.com",
            user_password: "senha123",
            expected_message: "O nome não pode estar vazio",
        },
        InvalidFieldTestCase {
            name: "email vazio",
            field_name: "email",
            user_name: "Teste",
            user_email: "",
            user_password: "senha123",
            expected_message: "O email não pode estar vazio",
        },
        InvalidFieldTestCase {
            name: "senha vazia",
            field_name: "senha",
            user_name: "Teste",
            user_email: "teste@example.com",
            user_password: "",
            expected_message: "deve ter pelo menos 6",
        },
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

        let dto = create_user_dto(
            test_case.user_name,
            test_case.user_email,
            test_case.user_password,
        );

        // Act
        let result = auth_service.register(dto).await;

        // Assert
        assert!(
            result.is_err(),
            "O registro deveria falhar com {} vazio",
            test_case.field_name
        );

        assert_validation_error_with_message(
            &result.unwrap_err(),
            test_case.expected_message,
            &format!("Erro ao validar {}", test_case.name),
        );
    }
}

/// Testa o registro de usuário com senha muito curta (menos de 6 caracteres)
///
/// Verifica se:
/// 1. O registro falha apropriadamente
/// 2. A mensagem de erro indica que a senha precisa ter pelo menos 6 caracteres
#[tokio::test]
async fn fail_with_short_password() {
    // Arrange
    let mock_repo = MockUserRepository::new();

    // Criar mock do PasswordService (não será usado devido à validação falhar antes)
    let mock_password_service = create_mock_password_service("hashed_password".to_string(), true);

    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_repo),
        mock_password_service,
        create_test_config(),
    );

    let dto = create_user_dto("Teste", "teste@example.com", "12345"); // Menos de 6 caracteres

    // Act
    let result = auth_service.register(dto).await;

    // Assert
    assert!(result.is_err(), "O registro deveria falhar com senha curta");

    assert_validation_error_with_message(
        &result.unwrap_err(),
        "A senha deve ter pelo menos 6 caracteres",
        "Erro ao validar tamanho mínimo da senha",
    );
}

/// Testa o registro de usuário com email já existente
///
/// Verifica se:
/// 1. O registro falha quando o email já está em uso
/// 2. A mensagem de erro indica especificamente que o email já está em uso
#[tokio::test]
async fn fail_with_duplicate_email() {
    // Arrange
    let mut mock_repo = MockUserRepository::new();

    // Simular erro de duplicação de email
    mock_repo
        .expect_create()
        .with(predicate::always(), predicate::always())
        .returning(|_, _| {
            Err(AppError::ValidationError(
                "Email já está em uso".to_string(),
            ))
        });

    // Criar mock do PasswordService
    let mock_password_service = create_mock_password_service("hashed_password".to_string(), true);

    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_repo),
        mock_password_service,
        create_test_config(),
    );

    let dto = create_user_dto("Teste", "email_existente@example.com", "senha123");

    // Act
    let result = auth_service.register(dto).await;

    // Assert
    assert!(
        result.is_err(),
        "O registro deveria falhar com email duplicado"
    );

    assert_validation_error_with_message(
        &result.unwrap_err(),
        "Email já está em uso",
        "Erro ao validar email duplicado",
    );
}

/// Testa o registro de usuário com nome muito longo
///
/// Verifica se:
/// 1. O registro falha quando o nome é excessivamente longo
/// 2. A mensagem de erro indica que o nome é muito longo
#[tokio::test]
async fn fail_with_long_name() {
    // Arrange
    let mock_repo = MockUserRepository::new();

    // Criar mock do PasswordService (não será usado devido à validação falhar antes)
    let mock_password_service = create_mock_password_service("hashed_password".to_string(), true);

    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_repo),
        mock_password_service,
        create_test_config(),
    );

    // Criar um nome com 256 caracteres (acima do limite)
    let long_name = "A".repeat(256);
    let dto = create_user_dto(&long_name, "teste@example.com", "senha123");

    // Act
    let result = auth_service.register(dto).await;

    // Assert
    assert!(
        result.is_err(),
        "O registro deveria falhar com nome muito longo"
    );

    assert_error_with_message(
        &result.unwrap_err(),
        "ValidationError",
        "O nome deve ter menos de 255 caracteres",
        "Erro ao validar nome com tamanho excessivo",
    );
}

/// Testa o comportamento quando o repositório retorna um erro de banco de dados
///
/// Verifica se:
/// 1. O erro do repositório é propagado corretamente pelo serviço
/// 2. A mensagem de erro é mantida intacta
#[tokio::test]
async fn fail_with_database_error() {
    // Arrange
    let mut mock_repo = MockUserRepository::new();

    // Configurar mock para retornar um erro de banco de dados
    mock_repo
        .expect_create()
        .with(predicate::always(), predicate::always())
        .returning(|_, _| {
            Err(AppError::DatabaseError(
                "Erro de conexão com o banco de dados".to_string(),
            ))
        });

    // Criar mock do PasswordService
    let mock_password_service = create_mock_password_service("hashed_password".to_string(), true);

    let auth_service = AuthServiceImpl::new(
        Arc::new(mock_repo),
        mock_password_service,
        create_test_config(),
    );

    let dto = create_user_dto("Teste", "teste@example.com", "senha123");

    // Act
    let result = auth_service.register(dto).await;

    // Assert
    assert!(
        result.is_err(),
        "O registro deveria falhar com erro de banco de dados"
    );

    assert_error_with_message(
        &result.unwrap_err(),
        "DatabaseError",
        "Erro de conexão com o banco de dados",
        "Erro ao lidar com falha do repositório",
    );
}
