# Используем официальный образ Rust
FROM rust:1.75-slim as builder

# Создаем директорию для приложения
WORKDIR /usr/src/app

# Копируем файлы с зависимостями
COPY Cargo.toml Cargo.lock ./

# Создаем пустой проект для кэширования зависимостей
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Копируем исходный код
COPY src ./src
COPY config.yaml ./

# Пересобираем с реальным кодом
RUN cargo build --release

# Создаем финальный образ
FROM debian:bookworm-slim

# Устанавливаем CA certificates для HTTPS
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Копируем исполняемый файл и конфиг
COPY --from=builder /usr/src/app/target/release/better_call_put /usr/local/bin/
COPY --from=builder /usr/src/app/config.yaml /usr/local/bin/

WORKDIR /usr/local/bin

# Запускаем приложение
CMD ["better_call_put"] 