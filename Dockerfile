FROM rust:1.59.0

ARG IPFS_KEY
ENV IPFS_API_KEY ${IPFS_KEY}

RUN apt update && apt install lld clang -y

WORKDIR /usr/src/dbio-protocol
COPY . .
RUN cargo clean
RUN SQLX_OFFLINE=true cargo build --release

EXPOSE 8080

ENTRYPOINT ["./target/release/dbio-protocol"]
