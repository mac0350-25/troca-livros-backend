FROM postgres:14

# Copiar o script SQL para o diretório de inicialização
COPY ./setup.sql /docker-entrypoint-initdb.d/

# Copiar nosso script de entrypoint personalizado
COPY ./docker-entrypoint.sh /usr/local/bin/custom-entrypoint.sh

# Tornar o script executável
RUN chmod +x /usr/local/bin/custom-entrypoint.sh

EXPOSE 5432

# Usar nosso entrypoint personalizado
ENTRYPOINT ["/usr/local/bin/custom-entrypoint.sh"]
