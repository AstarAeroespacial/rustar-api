FROM rust:1.89-bookworm AS builder
WORKDIR /app

# Copy manifests first to maximize caching
COPY Cargo.toml Cargo.lock ./

# Copy sources and migrations (migrations are embedded by sqlx::migrate!)
COPY src ./src
COPY migrations ./migrations

COPY .sqlx ./.sqlx

# build
ENV SQLX_OFFLINE=true
RUN cargo build --release

# run
FROM debian:trixie-slim

WORKDIR /app

COPY --from=builder /app/target/release/api /usr/local/bin/

COPY config.toml /app/config.toml

ENV RUST_LOG=info

CMD ["api"]