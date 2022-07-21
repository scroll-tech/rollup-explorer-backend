#!/bin/bash
set -uex

export RUST_BACKTRACE=full

function handle_submodules() {
  git submodule update --init --recursive
  if [ -z ${CI+x} ]; then git pull --recurse-submodules; fi
}

function db_migrate() {
    goose -dir "backend/third-paries/scroll/store/migrate/migrations" \
        postgres "postgres://scroll:scroll2022@localhost:5434/scroll?sslmode=disable" \
        up
}

function run() {
  docker compose -f docker-compose.yml up -d db --wait
  db_migrate
  cd backend && cargo run --bin rollup_explorer
}

function setup() {
  handle_submodules
}

setup
run