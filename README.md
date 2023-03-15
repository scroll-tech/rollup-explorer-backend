# Scroll's Rollup Explorer Backend

[![Main Test Status][test-image]][test-link]
[![Audit Status][audit-image]][audit-link]
![Rust Nightly][rustc-image]

## Purpose

This repo contains the backend code for the [Rollup Explorer](https://scroll.io/alpha/rollupscan) that's currently being maintained by Scroll.

## License

MIT.

## Contributing

If you encounter bugs or have feature ideas, feel free to [create an issue](/../../issues) or [write a PR](/../../pulls).

## Prerequisites

Naturally, you will need the [Rust toolchain] installed.
Besides that, [goose] is necessary for external database migrations in `database` dictionary of [scroll].

## ENV

- `BIND_PORT`: Internal binding HTTP port (`5001` as default).
- `DB_URL`: The database URL used to connect.
- `OPEN_API_ADDR`: Open API URL displayed on Web UI.
- `MAX_PER_PAGE`: Max value of query parameter `per_page` (100 as default)

## Development

- `make start`: Start a local `Postgres` docker-container, and `cargo run --bin rollup_explorer`. Then URL `http://0.0.0.0:5001` could be accessed in a Web browser.

- `make stop`: Stop running `rollup_explorer` processes and `Postgres` docker-container. The `Postgres` data should also be cleared via deleting folder `docker-data`.

- `make lint`: Format and lint codes.

- `make shfmt`: Format Shell scripts.

## Adding Mock Data

Run the following:

`psql postgres://postgres:scroll2022@localhost:5434/scroll -f db/tests/test.sql`

## Deployment

Currently deployed by Scroll's devops and used by https://scroll.io/alpha/rollupscan.

[//]: # "badges"
[Rust toolchain]: https://rustup.rs
[audit-image]: https://github.com/scroll-tech/rollup-explorer-backend/actions/workflows/audit.yml/badge.svg
[audit-link]: https://github.com/scroll-tech/rollup-explorer-backend/actions/workflows/audit.yml
[goose]: https://github.com/pressly/goose
[rustc-image]: https://img.shields.io/badge/rustc-nightly-blue.svg
[scroll]: https://github.com/scroll-tech/scroll
[test-image]: https://github.com/scroll-tech/rollup-explorer-backend/actions/workflows/test.yml/badge.svg
[test-link]: https://github.com/scroll-tech/rollup-explorer-backend/actions/workflows/test.yml
