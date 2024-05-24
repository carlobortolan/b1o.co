FROM rust:1.78.0-alpine3.18 AS builder
RUN apk upgrade --update-cache --available && \
    apk add build-base
COPY ./backend ./app/backend
COPY ./database ./app/database
COPY ./shared ./app/shared
COPY ./.env /app/.env
RUN cargo build --release --manifest-path /app/backend/Cargo.toml

FROM alpine:3.16
COPY --from=builder /app/backend/target/release /target/release/backend
COPY --from=builder /app/.env /.env
RUN ["chmod", "+x", "/target/release/backend"]
EXPOSE 8080

CMD ["/target/release/backend/backend"]