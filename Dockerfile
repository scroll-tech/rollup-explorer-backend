# Build

FROM scrolltech/rust-alpine-builder:nightly-2022-08-23 AS chef
WORKDIR app

FROM chef AS planner
RUN --mount=target=. \
    cargo chef prepare --recipe-path /recipe.json

FROM chef AS builder
COPY --from=planner /recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN --mount=target=. \
    cargo build --release --target-dir=/app-target

# Release

FROM alpine:3.15

WORKDIR /app

COPY --from=builder /app-target/release/rollup_explorer /bin/

ENTRYPOINT ["rollup_explorer"]
