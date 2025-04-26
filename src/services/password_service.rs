use async_trait::async_trait;
use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::error::AppError;

/// Interface para serviços de gerenciamento de senha
///
/// Esta interface define operações para hash e verificação de senhas,
/// permitindo desacoplar o AuthService da implementação específica (Argon2)
#[async_trait]
pub trait PasswordService: Send + Sync + 'static {
    /// Gera um hash para uma senha
    fn hash_password(&self, password: &str) -> Result<String, AppError>;

    /// Verifica se uma senha corresponde a um hash
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError>;
}

/// Implementação do PasswordService usando Argon2
pub struct Argon2PasswordService;

impl Argon2PasswordService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl PasswordService for Argon2PasswordService {
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
}

/// Factory para criar instâncias do PasswordService
///
/// Permite flexibilidade para mudar a implementação no futuro
pub fn create_password_service() -> Arc<dyn PasswordService> {
    Arc::new(Argon2PasswordService::new())
}
