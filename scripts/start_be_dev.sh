#!/bin/bash
set -uex

export RUST_BACKTRACE=full

docker compose -f docker-compose.yml up -d db --wait
cd backend && cargo run --bin rollup_explorer
