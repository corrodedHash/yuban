FROM rust as builder
RUN rustup install nightly

WORKDIR /usr/src/myapp
COPY . .
RUN cargo +nightly build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/yuban /usr/local/bin/yuban
RUN apt-get update
RUN apt-get install -y libssl-dev
ENTRYPOINT ["yuban", "--mysql-host", "yuban-db", "--bind-address", "0.0.0.0"]