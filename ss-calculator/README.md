# Secret Society Calculator

The calculator is a client-side Leptos/WASM app. Its rules and data parsing live in the pure Rust `secret-society-core` crate; shared UI components are in `bat-ui-components`.

## Prerequisites

- Rust and Cargo
- The `wasm32-unknown-unknown` Rust target
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

Install the WASM target if needed:

```sh
rustup target add wasm32-unknown-unknown
```

## Test

Run all workspace tests from the repository root:

```sh
cargo test --workspace
```

## Lint and format

Run these from the repository root:

```sh
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Build the WASM package

Run from the `ss-calculator/` directory:

```sh
wasm-pack build --target web --out-dir pkg --out-name ss_calculator --release
```

This writes the static browser package into `ss-calculator/pkg/`.

## Serve locally

Build the WASM package first. Then run a static server from the repository root:

```sh
python3 -m http.server 8000
```

Open <http://localhost:8000/ss-calculator/> in your browser.
