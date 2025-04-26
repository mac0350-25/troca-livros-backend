use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;
use crate::error::AppError;
use crate::models::user::{CreateUserDto, LoginUserDto, TokenResponse, UserResponse};
use crate::repositories::user_repository::UserRepository;
use crate::services::password_service::PasswordService;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[async_trait]
pub trait AuthService: Send + Sync + 'static {
    async fn register(&self, user_dto: CreateUserDto) -> Result<UserResponse, AppError>;
    async fn login(&self, login_dto: LoginUserDto) -> Result<TokenResponse, AppError>;
}

pub struct AuthServiceImpl {
    user_repository: Arc<dyn UserRepository>,
    password_service: Arc<dyn PasswordService>,
    config: Config,
}

impl AuthServiceImpl {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_service: Arc<dyn PasswordService>,
        config: Config,
    ) -> Self {
        Self {
            user_repository,
            password_service,
            config,
        }
    }

    fn generate_token(&self, user_id: &Uuid) -> Result<String, AppError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::hours(self.config.jwt_expires_in.parse::<i64>().unwrap()))
            .timestamp() as usize;

        let claims = TokenClaims {
            sub: user_id.to_string(),
            iat,
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("Erro ao gerar token: {}", e)))
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn register(&self, user_dto: CreateUserDto) -> Result<UserResponse, AppError> {
        // Validar entrada usando as validações do DTO
        user_dto.validate_all()?;

        // Hash da senha usando o adapter
        let hash_password = self.password_service.hash_password(&user_dto.password)?;

        // Criar usuário
        let user = self
            .user_repository
            .create(&user_dto, hash_password)
            .await?;

        Ok(UserResponse::from(user))
    }

    async fn login(&self, login_dto: LoginUserDto) -> Result<TokenResponse, AppError> {
        // Validar entrada usando as validações do DTO
        login_dto.validate_all()?;

        // Buscar usuário pelo email
        let user = self
            .user_repository
            .find_by_email(&login_dto.email)
            .await?
            .ok_or_else(|| AppError::AuthError("Credenciais inválidas".to_string()))?;

        // Verificar senha usando o adapter
        let is_valid = self
            .password_service
            .verify_password(&login_dto.password, &user.hash_password)?;
        if !is_valid {
            return Err(AppError::AuthError("Credenciais inválidas".to_string()));
        }

        // Gerar token
        let token = self.generate_token(&user.id)?;

        Ok(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            user: UserResponse::from(user),
        })
    }
}
