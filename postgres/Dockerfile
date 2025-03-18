FROM postgres:14

# Copiar apenas o script SQL para o diretório de inicialização
COPY ./setup.sql /docker-entrypoint-initdb.d/

EXPOSE 5432
