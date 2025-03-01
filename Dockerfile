# Stage 1: Build the Rust binary
FROM rust:1.67 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/ticket-assistant ./ticket-assistant

ENTRYPOINT ["./ticket-assistant"]
