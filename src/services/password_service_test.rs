use crate::error::AppError;
use crate::services::password_service::{
    create_password_service, Argon2PasswordService, PasswordService,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_success() {
        // Arrange
        let service = Argon2PasswordService::new();
        let password = "senha123";

        // Act
        let result = service.hash_password(password);

        // Assert
        assert!(
            result.is_ok(),
            "O hash da senha deveria ser gerado com sucesso"
        );
        let hash = result.unwrap();
        assert!(!hash.is_empty(), "O hash não deve ser vazio");
        assert!(
            hash.contains("$argon2"),
            "O hash deve usar o formato Argon2"
        );
    }

    #[test]
    fn test_verify_password_success() {
        // Arrange
        let service = Argon2PasswordService::new();
        let password = "senha123";

        // Primeiro, gerar um hash da senha
        let hash = service.hash_password(password).unwrap();

        // Act
        let result = service.verify_password(password, &hash);

        // Assert
        assert!(result.is_ok(), "A verificação deveria ser bem-sucedida");
        assert!(
            result.unwrap(),
            "A senha correta deveria ser verificada como válida"
        );
    }

    #[test]
    fn test_verify_password_with_wrong_password() {
        // Arrange
        let service = Argon2PasswordService::new();
        let correct_password = "senha123";
        let wrong_password = "senha456";

        // Primeiro, gerar um hash da senha correta
        let hash = service.hash_password(correct_password).unwrap();

        // Act
        let result = service.verify_password(wrong_password, &hash);

        // Assert
        assert!(
            result.is_ok(),
            "A verificação deveria ser processada sem erros"
        );
        assert!(
            !result.unwrap(),
            "A senha incorreta deveria ser verificada como inválida"
        );
    }

    #[test]
    fn test_verify_password_with_invalid_hash() {
        // Arrange
        let service = Argon2PasswordService::new();
        let password = "senha123";
        let invalid_hash = "hash_invalido";

        // Act
        let result = service.verify_password(password, invalid_hash);

        // Assert
        assert!(
            result.is_err(),
            "A verificação deveria falhar com hash inválido"
        );
        let error = result.unwrap_err();
        match error {
            AppError::InternalServerError(msg) => {
                assert!(
                    msg.contains("Erro ao analisar hash"),
                    "A mensagem de erro deveria indicar problema na análise do hash"
                );
            }
            _ => panic!("Tipo de erro inesperado: {:?}", error),
        }
    }

    #[test]
    fn test_create_password_service() {
        // Act
        let _service = create_password_service();

        // Assert
        // Verificamos se a função factory retorna corretamente um serviço
        // A verificação é implícita, pois se não retornasse um PasswordService,
        // o código não compilaria
        assert!(true, "O serviço foi criado com sucesso");
    }

    #[test]
    fn test_hashed_passwords_are_different() {
        // Arrange
        let service = Argon2PasswordService::new();
        let password = "senha123";

        // Act
        let hash1 = service.hash_password(password).unwrap();
        let hash2 = service.hash_password(password).unwrap();

        // Assert
        assert_ne!(
            hash1, hash2,
            "Hashes da mesma senha devem ser diferentes (salt diferente)"
        );
    }
}
