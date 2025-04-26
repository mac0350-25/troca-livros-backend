use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use validator::{validate_email, ValidationError};

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateUserDto {
    /// Nome do usuário (máximo 255 caracteres)
    #[validate(length(min = 1, message = "O nome não pode estar vazio"))]
    #[validate(length(max = 255, message = "O nome deve ter menos de 255 caracteres"))]
    pub name: String,

    /// Email do usuário (deve ser único e em formato válido)
    #[validate(length(min = 1, message = "O email não pode estar vazio"))]
    #[validate(length(max = 255, message = "O email deve ter menos de 255 caracteres"))]
    #[validate(custom = "validate_email_format")]
    pub email: String,

    /// Senha do usuário (mínimo 6 caracteres)
    #[validate(length(min = 6, message = "A senha deve ter pelo menos 6 caracteres"))]
    pub password: String,
}

impl CreateUserDto {
    /// Valida todos os campos do DTO
    ///
    /// Retorna erro se algum campo não estiver de acordo com as regras de validação
    pub fn validate_all(&self) -> Result<(), crate::error::AppError> {
        match self.validate() {
            Ok(_) => Ok(()),
            Err(e) => Err(crate::error::AppError::ValidationError(format!(
                "Erro de validação: {}",
                e
            ))),
        }
    }
}

/// Função personalizada para validar formato de email
fn validate_email_format(email: &str) -> Result<(), ValidationError> {
    if validate_email(email) {
        Ok(())
    } else {
        let mut error = ValidationError::new("email");
        error.message = Some("Formato de email inválido".into());
        Err(error)
    }
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LoginUserDto {
    /// Email do usuário
    #[validate(length(min = 1, message = "O email não pode estar vazio"))]
    #[validate(length(max = 255, message = "O email deve ter menos de 255 caracteres"))]
    #[validate(custom = "validate_email_format")]
    pub email: String,

    /// Senha do usuário
    #[validate(length(min = 1, message = "A senha não pode estar vazia"))]
    pub password: String,
}

impl LoginUserDto {
    /// Valida todos os campos do DTO
    ///
    /// Retorna erro se algum campo não estiver de acordo com as regras de validação
    pub fn validate_all(&self) -> Result<(), crate::error::AppError> {
        match self.validate() {
            Ok(_) => Ok(()),
            Err(e) => Err(crate::error::AppError::ValidationError(format!(
                "Erro de validação: {}",
                e
            ))),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    /// ID único do usuário
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    /// Nome do usuário
    pub name: String,
    /// Email do usuário
    pub email: String,
    /// Data de criação do registro
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    /// Data da última atualização
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    /// Token JWT de acesso
    pub access_token: String,
    /// Tipo do token (Bearer)
    pub token_type: String,
    /// Informações do usuário
    pub user: UserResponse,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
