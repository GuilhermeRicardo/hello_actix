FROM rust:latest

WORKDIR /usr/src/app

COPY . .

# Construir o projeto
RUN cargo build --release

# Adicionar permissão de execução ao binário
RUN chmod +x /usr/src/app/target/release/hello_actix

# Comando para iniciar o servidor
CMD ["cargo", "run", "--release"]

# Expor a porta 8080
EXPOSE 8080
