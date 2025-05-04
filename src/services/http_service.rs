use crate::error::AppError;
use reqwest::Client;
use serde_json::Value;

pub trait HttpService: Send + Sync {
    fn get<'a>(
        &'a self,
        url: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, AppError>> + Send + 'a>>;
}

pub struct HttpServiceImpl {
    client: Client,
}

impl HttpServiceImpl {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl HttpService for HttpServiceImpl {
    fn get<'a>(
        &'a self,
        url: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, AppError>> + Send + 'a>>
    {
        Box::pin(async move {
            let response = self.client.get(url).send().await.map_err(|e| {
                AppError::InternalServerError(format!("Erro na requisição HTTP: {}", e))
            })?;

            let status = response.status();
            if !status.is_success() {
                let message = format!("Erro na requisição: Status {}", status);
                return Err(AppError::NotFoundError(message));
            }

            let data: Value = response.json().await.map_err(|e| {
                AppError::InternalServerError(format!("Erro ao processar resposta: {}", e))
            })?;

            Ok(data)
        })
    }
}
