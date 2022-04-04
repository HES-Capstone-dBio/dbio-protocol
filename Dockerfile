FROM rust:1.59.0

RUN apt update && apt install lld clang -y
WORKDIR /usr/src/dbio-protocol
COPY . .
RUN rm -r target
RUN SQLX_OFFLINE=true cargo build --release

ENTRYPOINT ["./target/release/dbio-protocol"]
