FROM rust:1.84 AS builder

WORKDIR /app

COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/rust_app /usr/local/bin/rust_app

EXPOSE 9000

CMD ["rust_app"]