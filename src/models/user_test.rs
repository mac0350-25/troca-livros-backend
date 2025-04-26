#[cfg(test)]
mod tests {
    use crate::models::user::CreateUserDto;
    use validator::Validate;

    #[test]
    fn test_valid_user_dto() {
        let dto = CreateUserDto {
            name: "Teste".to_string(),
            email: "teste@example.com".to_string(),
            password: "senha123".to_string(),
        };

        assert!(
            dto.validate().is_ok(),
            "O DTO válido deveria passar na validação"
        );
    }

    #[test]
    fn test_empty_name() {
        let dto = CreateUserDto {
            name: "".to_string(),
            email: "teste@example.com".to_string(),
            password: "senha123".to_string(),
        };

        let result = dto.validate();
        assert!(
            result.is_err(),
            "O DTO com nome vazio deveria falhar na validação"
        );

        let error = result.unwrap_err();
        assert!(
            error.field_errors().contains_key("name"),
            "Erro deveria estar no campo 'name'"
        );
        assert!(
            error.field_errors().get("name").unwrap()[0]
                .message
                .as_ref()
                .unwrap()
                .contains("não pode estar vazio"),
            "Mensagem de erro deveria indicar que o nome não pode estar vazio"
        );
    }

    #[test]
    fn test_long_name() {
        // Criar um nome com 256 caracteres (acima do limite)
        let name = "A".repeat(256);

        let dto = CreateUserDto {
            name,
            email: "teste@example.com".to_string(),
            password: "senha123".to_string(),
        };

        let result = dto.validate();
        assert!(
            result.is_err(),
            "O DTO com nome longo demais deveria falhar na validação"
        );

        let error = result.unwrap_err();
        assert!(
            error.field_errors().contains_key("name"),
            "Erro deveria estar no campo 'name'"
        );
        assert!(
            error.field_errors().get("name").unwrap()[0]
                .message
                .as_ref()
                .unwrap()
                .contains("menos de 255"),
            "Mensagem de erro deveria indicar que o nome deve ter menos de 255 caracteres"
        );
    }

    #[test]
    fn test_long_email() {
        // Criar um nome com 256 caracteres (acima do limite)
        let email = "A".repeat(256);

        let dto = CreateUserDto {
            name: "Teste".to_string(),
            email,
            password: "senha123".to_string(),
        };

        let result = dto.validate();
        assert!(
            result.is_err(),
            "O DTO com email longo demais deveria falhar na validação"
        );

        let error = result.unwrap_err();
        assert!(
            error.field_errors().contains_key("email"),
            "Erro deveria estar no campo 'email'"
        );
        assert!(
            error.field_errors().get("email").unwrap()[0]
                .message
                .as_ref()
                .unwrap()
                .contains("menos de 255"),
            "Mensagem de erro deveria indicar que o email deve ter menos de 255 caracteres"
        );
    }

    #[test]
    fn test_empty_email() {
        let dto = CreateUserDto {
            name: "Teste".to_string(),
            email: "".to_string(),
            password: "senha123".to_string(),
        };

        let result = dto.validate();
        assert!(
            result.is_err(),
            "O DTO com email vazio deveria falhar na validação"
        );

        let error = result.unwrap_err();
        assert!(
            error.field_errors().contains_key("email"),
            "Erro deveria estar no campo 'email'"
        );
        assert!(
            error.field_errors().get("email").unwrap()[0]
                .message
                .as_ref()
                .unwrap()
                .contains("não pode estar vazio"),
            "Mensagem de erro deveria indicar que o email não pode estar vazio"
        );
    }

    #[test]
    fn test_invalid_email_format() {
        let dto = CreateUserDto {
            name: "Teste".to_string(),
            email: "email_invalido".to_string(),
            password: "senha123".to_string(),
        };

        let result = dto.validate();
        assert!(
            result.is_err(),
            "O DTO com formato de email inválido deveria falhar na validação"
        );

        let error = result.unwrap_err();
        assert!(
            error.field_errors().contains_key("email"),
            "Erro deveria estar no campo 'email'"
        );
        assert!(
            error.field_errors().get("email").unwrap()[0]
                .message
                .as_ref()
                .unwrap()
                .contains("Formato de email inválido"),
            "Mensagem de erro deveria indicar que o formato do email é inválido"
        );
    }

    #[test]
    fn test_short_password() {
        let dto = CreateUserDto {
            name: "Teste".to_string(),
            email: "teste@example.com".to_string(),
            password: "12345".to_string(), // Menos de 6 caracteres
        };

        let result = dto.validate();
        assert!(
            result.is_err(),
            "O DTO com senha curta demais deveria falhar na validação"
        );

        let error = result.unwrap_err();
        assert!(
            error.field_errors().contains_key("password"),
            "Erro deveria estar no campo 'password'"
        );
        assert!(
            error.field_errors().get("password").unwrap()[0]
                .message
                .as_ref()
                .unwrap()
                .contains("pelo menos 6 caracteres"),
            "Mensagem de erro deveria indicar que a senha deve ter pelo menos 6 caracteres"
        );
    }

    #[test]
    fn test_validate_all_method() {
        // Teste com DTO válido
        let valid_dto = CreateUserDto {
            name: "Teste".to_string(),
            email: "teste@example.com".to_string(),
            password: "senha123".to_string(),
        };
        assert!(
            valid_dto.validate_all().is_ok(),
            "O método validate_all deveria retornar Ok para um DTO válido"
        );

        // Teste com DTO inválido
        let invalid_dto = CreateUserDto {
            name: "".to_string(),
            email: "email_invalido".to_string(),
            password: "12345".to_string(),
        };
        let result = invalid_dto.validate_all();
        assert!(
            result.is_err(),
            "O método validate_all deveria retornar Err para um DTO inválido"
        );

        let error = result.unwrap_err();
        match error {
            crate::error::AppError::ValidationError(msg) => {
                assert!(
                    msg.contains("Erro de validação"),
                    "Mensagem de erro deveria começar com 'Erro de validação'"
                );
            }
            _ => panic!("Tipo de erro inesperado, esperava ValidationError"),
        }
    }
}
