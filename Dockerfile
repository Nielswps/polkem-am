FROM ubuntu:22.04

WORKDIR /bin

ADD --chown=777 target/release/account_manager .

VOLUME keys

EXPOSE 3000/tcp
ENTRYPOINT [ "account_manager" ]
