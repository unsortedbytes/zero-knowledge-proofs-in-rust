FROM rust:1.97

WORKDIR /zero-knowledge-proofs

COPY . .

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

RUN cargo build --release --bin server --bin client