FROM rust:1.47.0-slim as build

ENV PATH /root/.cargo/bin:$PATH
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev

COPY Cargo.lock /root
COPY Cargo.toml /root
COPY src /root/src

WORKDIR /root
RUN cargo build --release \
    && strip /root/target/release/geth-exporter-rust

## 2nd stage

FROM debian:buster-slim

WORKDIR /root

RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl1.1 \
    && apt-get -y clean \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p target/debug

COPY --from=build /root/target/release/geth-exporter-rust /usr/local/bin

ENV RUST_BACKTRACE 1

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/geth-exporter-rust"]
