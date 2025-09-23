FROM rust:1.86-slim-bookworm AS builder

WORKDIR /usr/src/

COPY . .
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/assets assets
COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/todo-cli todo-cli

ENTRYPOINT ["/usr/app/todo-cli"]
