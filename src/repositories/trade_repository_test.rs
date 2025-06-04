use crate::{
    repositories::{
        test_helpers::{get_test_db_pool, get_test_mutex},
        trade_repository::{PgTradeRepository, TradeRepository},
    },
};
use sqlx::PgPool;
use uuid::Uuid;

async fn setup_test_repository() -> PgTradeRepository {
    // Obtém o pool de conexão com o banco de dados de teste
    let pool = get_test_db_pool().await;
    
    // Criamos o PgTradeRepository com o pool real
    PgTradeRepository::new(pool)
}

async fn setup_test_data(pool: &PgPool) -> (Uuid, Uuid, Uuid, Uuid) {
    // Criar dois usuários de teste
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, name, email, hash_password) VALUES ($1, $2, $3, $4)",
        user1_id,
        "User 1",
        "user1@test.com",
        "hash1"
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO users (id, name, email, hash_password) VALUES ($1, $2, $3, $4)",
        user2_id,
        "User 2",
        "user2@test.com",
        "hash2"
    )
    .execute(pool)
    .await
    .unwrap();

    // Criar dois livros de teste
    let book1_id = Uuid::new_v4();
    let book2_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO books (id, title, author, description, image_url) VALUES ($1, $2, $3, $4, $5)",
        book1_id,
        "Livro 1",
        "Autor 1",
        "Descrição do Livro 1",
        "http://example.com/book1.jpg"
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO books (id, title, author, description, image_url) VALUES ($1, $2, $3, $4, $5)",
        book2_id,
        "Livro 2",
        "Autor 2",
        "Descrição do Livro 2",
        "http://example.com/book2.jpg"
    )
    .execute(pool)
    .await
    .unwrap();

    // User1 oferece Book1 e quer Book2
    sqlx::query!(
        "INSERT INTO books_offered (book_id, user_id) VALUES ($1, $2)",
        book1_id,
        user1_id
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO books_wanted (book_id, user_id) VALUES ($1, $2)",
        book2_id,
        user1_id
    )
    .execute(pool)
    .await
    .unwrap();

    // User2 oferece Book2 e quer Book1
    sqlx::query!(
        "INSERT INTO books_offered (book_id, user_id) VALUES ($1, $2)",
        book2_id,
        user2_id
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO books_wanted (book_id, user_id) VALUES ($1, $2)",
        book1_id,
        user2_id
    )
    .execute(pool)
    .await
    .unwrap();

    (user1_id, user2_id, book1_id, book2_id)
}

#[tokio::test]
async fn test_find_possible_trades_success() {
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let trade_repository = setup_test_repository().await;
    let pool = get_test_db_pool().await;

    let (user1_id, _user2_id, _book1_id, _book2_id) = setup_test_data(&pool).await;

    let result = trade_repository.find_possible_trades(user1_id).await;

    assert!(result.is_ok(), "Deve encontrar trocas possíveis com sucesso");
    let trades = result.unwrap();
    assert_eq!(trades.len(), 1, "Deve encontrar exatamente uma troca possível");

    let trade = &trades[0];
    assert_eq!(trade.offered_book.title, "Livro 1", "Livro oferecido deve ser 'Livro 1'");
    assert_eq!(trade.wanted_book.title, "Livro 2", "Livro desejado deve ser 'Livro 2'");
    assert_eq!(trade.trade_partner.name, "User 2", "Parceiro deve ser 'User 2'");
}

#[tokio::test]
async fn test_find_possible_trades_no_matches() {
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let trade_repository = setup_test_repository().await;
    let pool = get_test_db_pool().await;

    let user_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (id, name, email, hash_password) VALUES ($1, $2, $3, $4)",
        user_id,
        "User Lonely",
        "lonely@test.com",
        "hash"
    )
    .execute(&pool)
    .await
    .unwrap();

    let result = trade_repository.find_possible_trades(user_id).await;

    assert!(result.is_ok(), "Deve executar busca sem erros");
    let trades = result.unwrap();
    assert_eq!(trades.len(), 0, "Não deve encontrar trocas para usuário sem livros");
}

#[tokio::test]
async fn test_find_possible_trades_with_multiple_matches() {
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let trade_repository = setup_test_repository().await;
    let pool = get_test_db_pool().await;

    // Criar 3 usuários
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    let user3_id = Uuid::new_v4();

    for (id, name, email) in [
        (user1_id, "User 1", "user1@test.com"),
        (user2_id, "User 2", "user2@test.com"),
        (user3_id, "User 3", "user3@test.com"),
    ] {
        sqlx::query!(
            "INSERT INTO users (id, name, email, hash_password) VALUES ($1, $2, $3, $4)",
            id, name, email, "hash"
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    // Criar 3 livros
    let book1_id = Uuid::new_v4();
    let book2_id = Uuid::new_v4();
    let book3_id = Uuid::new_v4();

    for (id, title, author) in [
        (book1_id, "Livro 1", "Autor 1"),
        (book2_id, "Livro 2", "Autor 2"),
        (book3_id, "Livro 3", "Autor 3"),
    ] {
        sqlx::query!(
            "INSERT INTO books (id, title, author, description, image_url) VALUES ($1, $2, $3, $4, $5)",
            id, title, author, "Descrição", "http://example.com/book.jpg"
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    // User1 oferece Book1, quer Book2 e Book3
    sqlx::query!("INSERT INTO books_offered (book_id, user_id) VALUES ($1, $2)", book1_id, user1_id).execute(&pool).await.unwrap();
    sqlx::query!("INSERT INTO books_wanted (book_id, user_id) VALUES ($1, $2)", book2_id, user1_id).execute(&pool).await.unwrap();
    sqlx::query!("INSERT INTO books_wanted (book_id, user_id) VALUES ($1, $2)", book3_id, user1_id).execute(&pool).await.unwrap();

    // User2 oferece Book2, quer Book1
    sqlx::query!("INSERT INTO books_offered (book_id, user_id) VALUES ($1, $2)", book2_id, user2_id).execute(&pool).await.unwrap();
    sqlx::query!("INSERT INTO books_wanted (book_id, user_id) VALUES ($1, $2)", book1_id, user2_id).execute(&pool).await.unwrap();

    // User3 oferece Book3, quer Book1
    sqlx::query!("INSERT INTO books_offered (book_id, user_id) VALUES ($1, $2)", book3_id, user3_id).execute(&pool).await.unwrap();
    sqlx::query!("INSERT INTO books_wanted (book_id, user_id) VALUES ($1, $2)", book1_id, user3_id).execute(&pool).await.unwrap();

    let result = trade_repository.find_possible_trades(user1_id).await;

    assert!(result.is_ok(), "Deve encontrar múltiplas trocas com sucesso");
    let trades = result.unwrap();
    assert_eq!(trades.len(), 2, "Deve encontrar duas trocas possíveis");

    // Verificar que as trocas encontradas são válidas
    let trade_partners: Vec<&str> = trades.iter().map(|t| t.trade_partner.name.as_str()).collect();
    assert!(trade_partners.contains(&"User 2"), "Deve incluir User 2 como parceiro");
    assert!(trade_partners.contains(&"User 3"), "Deve incluir User 3 como parceiro");
}

#[tokio::test]
async fn test_find_possible_trades_excludes_same_user() {
    let mutex = get_test_mutex().await;
    let _lock = mutex.lock().await;

    let trade_repository = setup_test_repository().await;
    let pool = get_test_db_pool().await;

    let user_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (id, name, email, hash_password) VALUES ($1, $2, $3, $4)",
        user_id, "Self Trade User", "self@test.com", "hash"
    )
    .execute(&pool)
    .await
    .unwrap();

    let book1_id = Uuid::new_v4();
    let book2_id = Uuid::new_v4();

    for (id, title) in [(book1_id, "Book A"), (book2_id, "Book B")] {
        sqlx::query!(
            "INSERT INTO books (id, title, author, description, image_url) VALUES ($1, $2, $3, $4, $5)",
            id, title, "Author", "Description", "http://example.com/book.jpg"
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    // Usuário oferece Book A e quer Book B (não deve criar troca consigo mesmo)
    sqlx::query!("INSERT INTO books_offered (book_id, user_id) VALUES ($1, $2)", book1_id, user_id).execute(&pool).await.unwrap();
    sqlx::query!("INSERT INTO books_wanted (book_id, user_id) VALUES ($1, $2)", book2_id, user_id).execute(&pool).await.unwrap();

    let result = trade_repository.find_possible_trades(user_id).await;

    assert!(result.is_ok(), "Deve executar busca sem erros");
    let trades = result.unwrap();
    assert_eq!(trades.len(), 0, "Não deve encontrar troca do usuário consigo mesmo");
} 