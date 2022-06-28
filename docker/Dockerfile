# Build

FROM rustlang/rust:nightly-alpine3.15 as builder

RUN apk add --no-cache musl-dev

RUN mkdir -p /root/src
ADD . /root/src
RUN cd /root/src && cargo build --release

# Release

FROM alpine:3.15

ENV RUN_MODE "production"

RUN mkdir -p /root/config
COPY --from=builder /root/src/config/ /root/config/
COPY --from=builder /root/src/target/release/rollup_explorer /bin/

WORKDIR /root

ENTRYPOINT ["rollup_explorer"]
