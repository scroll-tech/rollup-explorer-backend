# Rollup Explorer

[![Main Test Status][test-image]][test-link]
[![Audit Status][audit-image]][audit-link]
![Rust Nightly][rustc-image]

## Pre-requisites

Naturally, you will need the [Rust toolchain] installed.
Besides that, [goose] is necessary for external database migrations of [scroll].

## Development

- `make start_be`: Start a local `Postgres` docker-container, and `cargo run --bin rollup_explorer`. Then URL `http://0.0.0.0:5000` could be accessed in a Web browser.

- `make start_fe`: Start the frontend. Then could visit it via [http://localhost:3000/](http://localhost:3000/).

- `make stop`: Stop running `rollup_explorer` processes and `Postgres` docker-container. The `Postgres` data should also be cleared via deleting folder `docker-data`.

- `make lint`: Format and lint codes.

- `make shfmt`: Format Shell scripts.


## Release

- Generate a docker image `scrolltech/rollup-explorer:latest` after running command `NEXT_PUBLIC_BASE_API_URL="http://44.239.40.223:8560/api" make docker`.

- This docker image could be tested via command `docker-compose -f docker/docker-compose.yml up -d`. Then access `http://0.0.0.0:5000` in a Web browser.

[//]: # "badges"
[Rust toolchain]: https://rustup.rs
[audit-image]: https://github.com/scroll-tech/rollup_explorer/actions/workflows/audit.yml/badge.svg
[audit-link]: https://github.com/scroll-tech/rollup_explorer/actions/workflows/audit.yml
[goose]: https://github.com/pressly/goose
[rustc-image]: https://img.shields.io/badge/rustc-nightly-blue.svg
[scroll]: https://github.com/scroll-tech/scroll
[test-image]: https://github.com/scroll-tech/rollup_explorer/actions/workflows/test.yml/badge.svg
[test-link]: https://github.com/scroll-tech/rollup_explorer/actions/workflows/test.yml

## Adding Mock Data

Run the following: 

`psql postgres://scroll:scroll2022@localhost:5434/rollup_explorer -f backend/db/tests/test.sql`
