FROM docker.io/clux/muslrust:1.68.2 as builder
RUN mkdir account-manager
WORKDIR ./account-manager

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release
RUN mv target/x86_64-unknown-linux-musl/release/account_manager /

FROM alpine:3.17.3
COPY --from=builder /account_manager /account_manager

RUN [ "./account_manager", "--version"]

EXPOSE 3000/tcp
VOLUME keys
ENTRYPOINT [ "./account_manager" ]
