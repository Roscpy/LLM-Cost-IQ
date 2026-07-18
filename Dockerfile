# Dockerfile
FROM rust:1.81-slim as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt update && apt install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/llm-cost-iq-backend /usr/local/bin/
EXPOSE 3000
CMD ["llm-cost-iq-backend"]
