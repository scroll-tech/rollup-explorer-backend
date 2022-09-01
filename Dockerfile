# Build

FROM scrolltech/rust-alpine-builder:latest as builder

RUN mkdir -p /root/src
ADD . /root/src
RUN cd /root/src && cargo build --release

# Release

FROM alpine:3.15

ENV OPEN_API_ADDR=$open_api_addr
ENV RUN_MODE="production"

RUN mkdir -p /root/config
COPY --from=builder /root/src/.env /root/
COPY --from=builder /root/src/config/ /root/config/
COPY --from=builder /root/src/target/release/rollup_explorer /bin/

WORKDIR /root

ENTRYPOINT ["rollup_explorer"]
