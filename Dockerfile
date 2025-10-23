FROM rust:1.89-bookworm AS base
RUN cargo install cargo-chef

FROM base AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# build dependencies
FROM base AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# build app
COPY . .
RUN cargo build --release --bin api


# run
FROM debian:trixie-slim
WORKDIR /app
COPY --from=builder /app/target/release/api /usr/local/bin/
COPY config.toml /app/config.toml
ENV RUST_LOG=info
CMD ["api"]