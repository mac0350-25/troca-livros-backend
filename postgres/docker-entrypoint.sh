#!/bin/bash
set -e

# Executar o entrypoint original do PostgreSQL
# Isso vai inicializar o banco de dados, criar usuários, etc.
docker-entrypoint.sh postgres "$@" &

# Aguarde o PostgreSQL estar pronto
until pg_isready -U "$POSTGRES_USER" -h localhost; do
    echo "Aguardando o PostgreSQL iniciar..."
    sleep 1
done

echo "PostgreSQL iniciado, executando script setup.sql..."

# Execute nosso script setup.sql
psql -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" -d "$POSTGRES_DB" -f /docker-entrypoint-initdb.d/setup.sql

echo "Script setup.sql executado com sucesso!"

# Mantenha o contêiner em execução com o PostgreSQL
wait 