#!/bin/bash
set -e

# Este script será executado após o PostgreSQL já estar inicializado
# porque está no diretório docker-entrypoint-initdb.d/

echo "Executando script de inicialização..."

# Executa o script SQL sempre que o contêiner iniciar
psql -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" -d "$POSTGRES_DB" -f /docker-entrypoint-initdb.d/setup.sql

echo "Script de inicialização executado com sucesso!" 