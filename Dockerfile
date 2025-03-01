# Stage 1: Build the Rust binary
FROM rust:latest AS builder

WORKDIR /ticket-assistant

COPY . .

RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /ticket-assistant/target/release/ticket-assistant /ticket-assistant

ENTRYPOINT ["/ticket-assistant"]
