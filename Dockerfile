FROM rust:latest as builder
WORKDIR /usr/local/prgm

ARG BIN
RUN test -n "$BIN"
COPY . .
RUN cargo build --release
RUN cargo install --path . --root /usr/local/install --bin $BIN

FROM ubuntu:latest
COPY --from=builder /usr/local/install/bin /usr/local/bin