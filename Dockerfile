FROM rust:1.59.0

WORKDIR /usr/src/dbio-protocol
COPY . .

RUN cargo install --path .

CMD ["cargo", "run", "--release"]
