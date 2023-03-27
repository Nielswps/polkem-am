FROM scratch

COPY target/release/main /main
COPY resources /resources

EXPOSE 3000

ENTRYPOINT ["/main"]
