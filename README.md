# Rollup Explorer
[![Main Test Status][test-image]][test-link]
[![Audit Status][audit-image]][audit-link]
![Rust Nightly][rustc-image]

## Development

- `make start`: Start a local `Postgres` docker-container, and `cargo run --bin rollup_explorer`. Then URL `http://0.0.0.0:5000` could be accessed in a Web browser.

- `make stop`: Stop running `rollup_explorer` process and `Postgres` docker-container. The `Postgres` data should also be cleared via deleting folder `docker-data`.

- `make lint`: Run both `cargo fmt` and `cargo clippy`.

- `make shfmt`: Format Shell scripts.

## Release

- Generate a docker image `scrolltech/rollup-explorer:latest` after running command `make docker`.

- This docker image could be tested via command `docker-compose -f docker/docker-compose.yml up -d`. Then access `http://0.0.0.0:5000` in a Web browser.

[//]: # (badges)

[audit-image]: https://github.com/scroll-tech/rollup_explorer/actions/workflows/audit.yml/badge.svg
[audit-link]: https://github.com/scroll-tech/rollup_explorer/actions/workflows/audit.yml
[rustc-image]: https://img.shields.io/badge/rustc-nightly-blue.svg
[test-image]: https://github.com/scroll-tech/rollup_explorer/actions/workflows/test.yml/badge.svg
[test-link]: https://github.com/scroll-tech/rollup_explorer/actions/workflows/test.yml
