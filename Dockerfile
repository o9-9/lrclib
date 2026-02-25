FROM docker.io/rust:1.91-alpine3.22 as builder
RUN apk add --no-cache alpine-sdk
WORKDIR /usr/src/lrclib-rs
COPY . .
RUN cargo build --release --workspace

FROM alpine:3.22
RUN apk add --no-cache sqlite
COPY --from=builder /usr/src/lrclib-rs/target/release/lrclib /usr/local/bin/lrclib
RUN mkdir /data
EXPOSE 3300
CMD ["lrclib", "serve", "--port", "3300", "--database", "/data/db.sqlite3"]
