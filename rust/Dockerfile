ARG RUST_BUILDER_IMAGE=rust:1.96.0-alpine3.22
ARG RUNTIME_IMAGE=alpine:3.22.1
FROM ${RUST_BUILDER_IMAGE} AS builder

RUN apk add --no-cache cmake make musl-dev perl
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM ${RUNTIME_IMAGE}

ARG VCS_REF
LABEL org.opencontainers.image.revision=$VCS_REF
RUN apk add --no-cache ca-certificates \
    && addgroup -g 10001 -S app \
    && adduser -u 10001 -S -D -H -G app app
COPY --from=builder /build/target/release/taoyangli-tools /usr/local/bin/taoyangli-tools

USER 10001:10001
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/taoyangli-tools"]
