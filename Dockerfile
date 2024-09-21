FROM gcr.io/distroless/cc-debian12
ARG BINARY_PATH=./target/release
COPY ${BINARY_PATH}/zappy .
ENTRYPOINT ["./zappy"]
