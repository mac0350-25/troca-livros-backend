# 📚 Troca Livros Backend

Backend do projeto **Troca Livros**, desenvolvido em **Rust** utilizando o framework **Axum**.


---

## 🛠 Configuração do Banco de Dados com Docker

### 📌 Pré-requisitos
- Docker instalado
- Docker Compose instalado

### ⚙️ Configuração do Banco de Dados
1. Copie o arquivo .env.example e renomeie para .env
2. Inicie o banco de dados com Docker Compose:
   ```sh
   docker compose up -d --build
   ```
3. Verifique se o container está rodando corretamente:
   ```sh
   docker ps
   ```
   Você deverá ver um container chamado `troca-livros-postgres` na lista.

4. Para acessar o banco de dados via linha de comando:
   ```sh
   docker exec -it troca-livros-postgres psql -U admin -d troca_livros
   ```

5. Para verificar as tabelas criadas:
   ```sql
   \dt
   ```

6. Para parar o container do banco de dados:
   ```sh
   docker compose down
   ```

7. Para reiniciar completamente o banco de dados (remove todos os dados):
   ```sh
   docker compose down -v
   ```

---

## 🏛 Estrutura do Banco de Dados
O banco de dados contém as seguintes tabelas:

| Tabela          | Descrição                              |
|----------------|--------------------------------------|
| `users`        | Armazena informações dos usuários    |
| `books`        | Catálogo de livros                   |
| `books_wanted` | Livros que os usuários desejam       |
| `books_offered`| Livros que os usuários oferecem      |
| `trades`       | Registros de trocas entre usuários   |

---

## 🛠 Solução de Problemas

- Se o banco de dados não inicializar corretamente, verifique os logs:
  ```sh
  docker logs troca-livros-postgres
  ```
- Certifique-se de que as variáveis de ambiente no arquivo `.env` estão configuradas corretamente.
- Para resolver problemas de permissão:
  ```sh
  chmod -R 777 ./postgres
  ```

---

## Inicializando o servidor:
```bash
cargo run
```

## 📚 Documentação da API

A documentação da API está disponível através do Swagger UI:

- **URL**: http://localhost:50001/docs
- **OpenAPI JSON**: http://localhost:50001/api-docs/openapi.json

A documentação inclui todos os endpoints disponíveis, modelos de dados, parâmetros de requisição e respostas.

---

## 🧪 Executando Testes e Cobertura

### 📌 Pré-requisitos
- Rust e Cargo instalados
- Docker e Docker Compose instalados (para o banco de dados de testes)
- Para cobertura: instale a ferramenta Tarpaulin:
  ```bash
   cargo install cargo-tarpaulin
  ```

### 🏦 Banco de Dados para Testes
Para executar os testes, é necessário ter o banco de dados de testes em execução:

1. Certifique-se de que as variáveis de ambiente para o banco de testes estão configuradas no arquivo `.env`
2. Inicie o banco de dados de testes junto com o banco principal:
   ```sh
   docker compose up -d --build
   ```
3. Verifique se o container de testes está rodando:
   ```sh
   docker ps
   ```
   Você deverá ver um container chamado `troca-livros-postgres-test` na lista.

4. Para acessar o banco de dados de testes:
   ```sh
   docker exec -it troca-livros-postgres-test psql -U admin -d test_db
   ```

### 🧪 Executando Testes
Para rodar todos os testes do projeto:
```bash
cargo test
```

Para executar um teste específico:
```bash
cargo test nome_do_teste
```

Para executar todos os testes de um módulo específico:
```bash
cargo test nome_do_modulo
```

Exemplo:
```bash
# Executar testes do serviço de autenticação
cargo test services::auth_service_test

# Executar testes do serviço de consulta ao Google Books
cargo test services::google_book_service_test::tests
```

### 📊 Gerando Relatório de Cobertura
Para verificar a cobertura de testes:

1. Gere o relatório de cobertura em HTML:
   ```bash
   cargo tarpaulin --config tarpaulin.toml
   ```

2. Abra o relatório no navegador:
   O relatório será gerado no arquivo `tarpaulin-report.html`. Abra o arquivo em um navegador para visualizar a cobertura detalhada por arquivo e linha.