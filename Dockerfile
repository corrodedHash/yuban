FROM rust as builder
RUN rustup install nightly

WORKDIR /usr/src/myapp
COPY . .
RUN cargo +nightly build --release

FROM debian:buster-slim
RUN apt-get update
RUN apt-get install -y libssl-dev
COPY --from=builder /usr/src/myapp/target/release/yuban /usr/local/bin/yuban
# COPY yuban /usr/local/bin/yuban
COPY --chmod=0755 docker-entrypoint.sh /docker-entrypoint.sh
ENTRYPOINT ["bash", "/docker-entrypoint.sh"]