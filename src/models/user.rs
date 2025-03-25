use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use utoipa::ToSchema;

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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserDto {
    /// Nome do usuário
    pub name: String,
    /// Email do usuário (deve ser único)
    pub email: String,
    /// Senha do usuário (mínimo 6 caracteres)
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginUserDto {
    /// Email do usuário
    pub email: String,
    /// Senha do usuário
    pub password: String,
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