# webcomponents

Rust webcomponents powered by Askama and Tailwind CSS.

## Usage

### Import the library

Add the following to your `Cargo.toml`:

```toml
[dependencies]
webcomponents = { git = "https://github.com/alexandervantrijffel/webcomponents.git", package = "webcomponents" }
```

### Running locally

When developing web components, quick feedback on changes to the components is essential. This workspace contains the `crates/bin/serve` which supports development of the webcomponents by:

- Hosting of the webcomponents with an Axum webserver
- Providing optimal incremental build times (around 0.63s on a MacBook Pro M3 MAX)
- Live reload of the page with systemfd and live.js
- Generation and hosting of Tailwind CSS styles

Install [Just](https://github.com/casey/just) and [systemfd](https://github.com/mitsuhiko/systemfd) and run the following command:

```shell
just pnpm-dev & serve-watch
```

The web app is accessible at `http://localhost:8181`.

## Notes

- To optimize compilation times, debug info is disabled in the dev profile in [Cargo.toml](Cargo.toml).
