FROM gcr.io/distroless/cc@sha256:f81e5db8287d66b012d874a6f7fea8da5b96d9cc509aa5a9b5d095a604d4bca1

COPY target/x86_64-unknown-linux-gnu/release/aggregator_backend .
ENTRYPOINT ["./aggregator_backend"]
EXPOSE 8888/tcp
