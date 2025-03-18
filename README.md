# Troca Livros Backend
Backend do projeto Troca Livros, feito em Rust com o framework Axum.
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
