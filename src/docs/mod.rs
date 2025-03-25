pub mod auth_docs;

use utoipa::OpenApi;
use crate::models::user::{CreateUserDto, LoginUserDto, TokenResponse, UserResponse};


#[derive(OpenApi)]
#[openapi(
    paths(
        crate::docs::auth_docs::register,
        crate::docs::auth_docs::login,
    ),
    components(
        schemas(CreateUserDto, LoginUserDto, TokenResponse, UserResponse)
    ),
    tags(
        (name = "auth", description = "API de autenticação")
    ),
    info(
        title = "API Troca Livros",
        version = "1.0.0",
        description = "API para o sistema de troca de livros",
    )
)]
pub struct ApiDoc; 