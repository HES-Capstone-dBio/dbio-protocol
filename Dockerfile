# Use Rust 1.59
FROM rust:1.59.0

WORKDIR /usr/src/dbio-protocol
COPY . .

RUN cargo install --path .

CMD ["dbio-protocol"]