FROM docker.io/rust:1.68.0 as builder
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder ./target/release/account_manager /

RUN [ "./account_manager", "--version"]

EXPOSE 3000/tcp
VOLUME keys
ENTRYPOINT [ "./account_manager" ]
