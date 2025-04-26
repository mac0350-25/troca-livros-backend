use std::net::TcpListener;
use troca_livros_api::app;

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

    println!("Conectando ao banco de teste: {}", test_db_url);

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
