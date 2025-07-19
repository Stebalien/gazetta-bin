# Build stage
FROM rust:alpine as builder
RUN apk add --no-cache build-base
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bin gazetta

# Final stage
FROM alpine:latest
RUN apk add --no-cache bash findutils gzip brotli
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/gazetta /usr/local/bin/
ENTRYPOINT ["gazetta"]
