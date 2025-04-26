use async_trait::async_trait;
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::user::{CreateUserDto, User};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: &CreateUserDto, hash_password: String) -> Result<User, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(&self, user: &CreateUserDto, hash_password: String) -> Result<User, AppError> {
        let result = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email, hash_password) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&hash_password)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                AppError::ValidationError("Email já está em uso".to_string())
            } else {
                AppError::DatabaseError(e.to_string())
            }
        })?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::from)?;

        Ok(result)
    }
}
