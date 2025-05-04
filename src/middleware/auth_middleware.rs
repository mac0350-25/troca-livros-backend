use std::sync::Arc;

use axum::{
    extract::Extension,
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

use crate::{
    config::Config,
    error::AppError,
    services::auth_service::{AuthServiceImpl, TokenClaims},
};

pub async fn auth_middleware<B>(
    Extension(_auth_service): Extension<Arc<AuthServiceImpl>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    // Verificar a existência do token de autenticação no header Authorization
    let headers = request.headers();
    let auth_header = headers.get(header::AUTHORIZATION);
    let auth_header = auth_header
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::AuthError("Token de autenticação ausente".to_string()))?;

    // Validar o formato do token (Bearer <token>)
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::AuthError(
            "Formato de token inválido. Use Bearer <token>".to_string(),
        ));
    }

    // Extrair o token
    let token = auth_header.trim_start_matches("Bearer ").trim();

    // Carregar configuração
    let config = Config::from_env().expect("Falha ao carregar configuração");

    // Validar o token JWT
    let token_data = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::AuthError(format!("Token inválido: {}", e)))?;

    // Extrair o user_id do token e converter para Uuid
    let user_id_str = token_data.claims.sub;
    let user_id = Uuid::parse_str(&user_id_str)
        .map_err(|_| AppError::AuthError("ID de usuário inválido no token".to_string()))?;

    // Adicionar o user_id aos extensions para que as rotas possam acessá-lo
    request.extensions_mut().insert(user_id);

    // Passar a requisição para o próximo handler
    Ok(next.run(request).await)
}
