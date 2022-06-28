#!/bin/bash
set -uex

export RUST_BACKTRACE=full

docker-compose -f docker/docker-compose.yml up -d db --wait
cargo run --bin rollup_explorer
