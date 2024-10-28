FROM rust:1.70-slim as builder
WORKDIR /usr/src/whoamifuck
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y systemctl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/whoamifuck/target/release/whoamifuck /usr/local/bin/whoamifuck
ENTRYPOINT ["whoamifuck"]
