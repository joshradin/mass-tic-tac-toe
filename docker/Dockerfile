FROM rust:latest as builder

WORKDIR /usr/local/prgm
COPY .. .

RUN cargo install --path . --root /usr/local/install

FROM ubuntu:latest
COPY --from=builder /usr/local/install/bin /usr/local/bin
