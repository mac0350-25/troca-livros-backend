use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;
use crate::error::AppError;
use crate::models::user::{CreateUserDto, LoginUserDto, TokenResponse, User, UserResponse};
use crate::repositories::user_repository::UserRepository;

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
    #[allow(unused)]
    async fn get_user_from_token(&self, token: &str) -> Result<User, AppError>;
}

pub struct AuthServiceImpl {
    user_repository: Arc<dyn UserRepository>,
    config: Config,
}

impl AuthServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>, config: Config) -> Self {
        Self {
            user_repository,
            config,
        }
    }

    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AppError::InternalServerError(format!("Erro ao gerar hash: {}", e)))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::InternalServerError(format!("Erro ao analisar hash: {}", e)))?;
            
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    fn generate_token(&self, user_id: &Uuid) -> Result<String, AppError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::hours(self.config.jwt_expires_in.parse::<i64>().unwrap())).timestamp() as usize;

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
        // Validar entrada
        if user_dto.name.is_empty() || user_dto.email.is_empty() || user_dto.password.is_empty() {
            return Err(AppError::ValidationError(
                "Nome, email e senha são obrigatórios".to_string(),
            ));
        }

        if user_dto.password.len() < 6 {
            return Err(AppError::ValidationError(
                "A senha deve ter pelo menos 6 caracteres".to_string(),
            ));
        }

        // Hash da senha
        let hash_password = self.hash_password(&user_dto.password)?;

        // Criar usuário
        let user = self.user_repository.create(&user_dto, hash_password).await?;

        Ok(UserResponse::from(user))
    }

    async fn login(&self, login_dto: LoginUserDto) -> Result<TokenResponse, AppError> {
        // Buscar usuário pelo email
        let user = self
            .user_repository
            .find_by_email(&login_dto.email)
            .await?
            .ok_or_else(|| AppError::AuthError("Credenciais inválidas".to_string()))?;

        // Verificar senha
        let is_valid = self.verify_password(&login_dto.password, &user.hash_password)?;
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

    async fn get_user_from_token(&self, token: &str) -> Result<User, AppError> {
        // Decodificar token
        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::AuthError("Token inválido".to_string()))?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| AppError::AuthError("Token inválido".to_string()))?;

        // Buscar usuário pelo ID
        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Usuário não encontrado".to_string()))?;

        Ok(user)
    }
} 