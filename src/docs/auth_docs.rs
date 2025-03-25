use axum::Json;

use crate::error::AppError;
use crate::models::user::{CreateUserDto, LoginUserDto, TokenResponse, UserResponse};

/// Registra um novo usuário
///
/// Cria um novo usuário no sistema com os dados fornecidos.
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = CreateUserDto,
    responses(
        (status = 201, description = "Usuário registrado com sucesso", body = UserResponse),
        (status = 400, description = "Dados de entrada inválidos"),
        (status = 500, description = "Erro interno do servidor")
    ),
    tag = "auth"
)]
#[allow(unused)]
pub async fn register(_body: Json<CreateUserDto>) -> Result<Json<UserResponse>, AppError> {
    // Esta função é apenas para documentação
    unimplemented!()
}

/// Realiza login de usuário
///
/// Autentica um usuário e retorna um token JWT.
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginUserDto,
    responses(
        (status = 200, description = "Login realizado com sucesso", body = TokenResponse),
        (status = 401, description = "Credenciais inválidas"),
        (status = 500, description = "Erro interno do servidor")
    ),
    tag = "auth"
)]
#[allow(unused)]
pub async fn login(_body: Json<LoginUserDto>) -> Result<Json<TokenResponse>, AppError> {
    // Esta função é apenas para documentação
    unimplemented!()
} 