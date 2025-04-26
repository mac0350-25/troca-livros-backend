use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::error::AppError;
use crate::models::user::{CreateUserDto, LoginUserDto};
use crate::services::auth_service::AuthService;

pub struct AuthHandler {
    auth_service: Arc<dyn AuthService>,
}

impl AuthHandler {
    pub fn new(auth_service: Arc<dyn AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn register(
        &self,
        Json(create_user_dto): Json<CreateUserDto>,
    ) -> Result<impl IntoResponse, AppError> {
        let user = self.auth_service.register(create_user_dto).await?;

        Ok((
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "message": "Usuário registrado com sucesso",
                "data": user
            })),
        ))
    }

    pub async fn login(
        &self,
        Json(login_dto): Json<LoginUserDto>,
    ) -> Result<impl IntoResponse, AppError> {
        let token = self.auth_service.login(login_dto).await?;

        Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Login realizado com sucesso",
                "data": token
            })),
        ))
    }
}
