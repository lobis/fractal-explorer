## Fractal Explorer

[![build](https://github.com/lobis/fractal-explorer/actions/workflows/rust.yml/badge.svg)](https://github.com/lobis/fractal-explorer/actions/workflows/rust.yml)

A simple Julia Set explorer built using Rust.

### [Demo](https://lobis.github.io/fractal-explorer/)

### Build

To build the executable run:

```
cargo build --release
```

The build the web application run the following. You may need to install `wasm-pack` via `cargo install wasm-pack`.

```
wasm-pack build --target web
```
