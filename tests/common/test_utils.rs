use reqwest::Client;
use serde_json::{json, Value};
use std::net::TcpListener;
use troca_livros_api::app;
use uuid::Uuid;

pub struct TestApp {
    pub port: u16,
}

/// Configura um aplicativo de teste com um banco de dados de teste
///
/// Esta função:
/// 1. Cria uma conexão com o banco de dados de teste
/// 2. Configura o servidor Axum
/// 3. Inicia o servidor em uma porta aleatória
/// 4. Retorna o objeto TestApp com informações para os testes
pub async fn setup_test_app() -> TestApp {
    // Carregar variáveis de ambiente
    dotenv::dotenv().ok();

    // Configurar logging para testes
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter("info")
        .try_init();

    // Obter a URL do banco de dados de teste a partir da variável de ambiente
    let test_db_url =
        std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL não está definida");

    // Encontrar uma porta disponível
    let listener = TcpListener::bind("127.0.0.1:0").expect("Falha ao vincular a porta aleatória");
    let port = listener.local_addr().unwrap().port();

    // Configurar rotas sem Swagger UI (não necessário para testes)
    let created_app = app::create_app(&test_db_url).await;

    // Iniciar o servidor em uma nova thread
    let server = axum::Server::from_tcp(listener)
        .expect("Falha ao criar servidor a partir do listener")
        .serve(created_app.into_make_service());

    tokio::spawn(server);

    TestApp { port }
}

/// Cria um usuário de teste e retorna o token de autenticação
///
/// Esta função:
/// 1. Registra um novo usuário (se necessário)
/// 2. Faz login para obter o token JWT
/// 3. Retorna o token para ser usado em requisições autenticadas
#[allow(dead_code)]
pub async fn get_auth_token(app: &TestApp) -> String {
    let client = Client::new();

    // Criar credenciais únicas usando UUID para garantir unicidade absoluta
    // mesmo quando chamado no mesmo milissegundo
    let uuid = Uuid::new_v4().to_string();
    let email = format!("auth_test_{}@example.com", uuid);
    let password = "Senha@123";
    let name = format!("Usuário de Teste {}", uuid);

    // Registrar usuário
    let register_response = client
        .post(&format!("http://localhost:{}/api/auth/register", app.port))
        .json(&json!({
            "name": name,
            "email": email,
            "password": password
        }))
        .send()
        .await
        .expect("Falha ao registrar usuário de teste");

    assert_eq!(register_response.status(), reqwest::StatusCode::CREATED);

    // Fazer login
    let login_response = client
        .post(&format!("http://localhost:{}/api/auth/login", app.port))
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .expect("Falha ao autenticar usuário de teste");

    let login_body: Value = login_response
        .json()
        .await
        .expect("Falha ao ler resposta de login");

    // Extrair token
    login_body["data"]["access_token"]
        .as_str()
        .expect("Token não encontrado na resposta")
        .to_string()
}
