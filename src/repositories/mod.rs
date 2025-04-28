pub mod book_repository;
pub mod user_repository;

#[cfg(test)]
pub mod user_repository_test;

#[cfg(test)]
pub mod book_repository_test;

#[cfg(test)]
pub mod test_helpers {
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Mutex para garantir que apenas um teste por vez acesse o banco
    static TEST_MUTEX: tokio::sync::OnceCell<Arc<Mutex<()>>> = tokio::sync::OnceCell::const_new();

    // Retorna um mutex para garantir a execução sequencial dos testes
    pub async fn get_test_mutex() -> Arc<Mutex<()>> {
        TEST_MUTEX
            .get_or_init(|| async { Arc::new(Mutex::new(())) })
            .await
            .clone()
    }

    // Configura e retorna um pool de conexão com o banco de dados de teste
    pub async fn get_test_db_pool() -> PgPool {
        // Carrega variáveis de ambiente
        dotenv().ok();

        // Usa as credenciais do banco de dados de teste
        let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER deve estar definido");
        let db_password =
            env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD deve estar definido");
        let db_name = env::var("POSTGRES_TEST_DB").expect("POSTGRES_TEST_DB deve estar definido");
        let db_port =
            env::var("POSTGRES_TEST_PORT").expect("POSTGRES_TEST_PORT deve estar definido");

        // Constrói a string de conexão para o banco de dados de teste
        let connection_string = format!(
            "postgres://{}:{}@localhost:{}/{}",
            db_user, db_password, db_port, db_name
        );

        // Conecta ao banco de dados de teste
        let pool = PgPool::connect(&connection_string)
            .await
            .expect("Falha ao conectar ao banco de teste");

        clean_database(&pool).await;

        pool
    }

    // Limpa todas as tabelas do banco de teste para garantir um estado inicial conhecido
    pub async fn clean_database(pool: &PgPool) {
        // Limpa todas as tabelas que possam afetar o teste
        sqlx::query("TRUNCATE TABLE users CASCADE")
            .execute(pool)
            .await
            .expect("Falha ao limpar a tabela users");

        // Limpa outras tabelas relacionadas
        sqlx::query("TRUNCATE TABLE books_wanted CASCADE")
            .execute(pool)
            .await
            .expect("Falha ao limpar a tabela books_wanted");

        sqlx::query("TRUNCATE TABLE books_offered CASCADE")
            .execute(pool)
            .await
            .expect("Falha ao limpar a tabela books_offered");

        sqlx::query("TRUNCATE TABLE trades CASCADE")
            .execute(pool)
            .await
            .expect("Falha ao limpar a tabela trades");

        sqlx::query("TRUNCATE TABLE books CASCADE")
            .execute(pool)
            .await
            .expect("Falha ao limpar a tabela books");
    }
}
