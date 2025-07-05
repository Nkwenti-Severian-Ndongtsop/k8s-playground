FROM rust:1.88 as builder
WORKDIR /app

COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN cargo build --release && strip target/release/backend

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*


COPY --from=builder /app/target/release/backend .

EXPOSE 8080
CMD ["./backend"]
