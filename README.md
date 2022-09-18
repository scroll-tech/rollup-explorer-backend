# Rollup Explorer Backend

[![Main Test Status][test-image]][test-link]
[![Audit Status][audit-image]][audit-link]
![Rust Nightly][rustc-image]

## Pre-requisites

Naturally, you will need the [Rust toolchain] installed.
Besides that, [goose] is necessary for external database migrations of [scroll].

## Development

- `make start`:
Start a local `Postgres` docker-container, and `cargo run --bin rollup_explorer`. Then URL `http://0.0.0.0:5001` could be accessed in a Web browser.

- `make stop`:
Stop running `rollup_explorer` processes and `Postgres` docker-container. The `Postgres` data should also be cleared via deleting folder `docker-data`.

- `make import_mock_data`:
Import mock data to DB.

- `make lint`:
Format and lint codes.

- `make login_db`:
Login to local DB.

- `make shfmt`:
Format Shell scripts.

[//]: # "badges"
[Rust toolchain]: https://rustup.rs
[audit-image]: https://github.com/scroll-tech/rollup-explorer-backend/actions/workflows/audit.yml/badge.svg
[audit-link]: https://github.com/scroll-tech/rollup-explorer-backend/actions/workflows/audit.yml
[goose]: https://github.com/pressly/goose
[rustc-image]: https://img.shields.io/badge/rustc-nightly-blue.svg
[scroll]: https://github.com/scroll-tech/scroll
[test-image]: https://github.com/scroll-tech/rollup-explorer-backend/actions/workflows/test.yml/badge.svg
[test-link]: https://github.com/scroll-tech/rollup-explorer-backend/actions/workflows/test.yml
