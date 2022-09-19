# [Fractal Explorer](https://lobis.github.io/fractal-explorer/)

[![build](https://github.com/lobis/fractal-explorer/actions/workflows/rust.yml/badge.svg)](https://github.com/lobis/fractal-explorer/actions/workflows/rust.yml)

A simple Julia Set explorer built using Rust. This was created as a learning project and is based on [this great tutorial](https://sotrh.github.io/learn-wgpu/).

## [Live Demo](https://lobis.github.io/fractal-explorer/)

## Build

To build the executable run:

```
cargo build --release
```

To build the web application run the following command. You may need to install `wasm-pack` via `cargo install wasm-pack`.

```
wasm-pack build --target web --out-dir public/pkg
```

All files related to the static site will be placed under `public` with `index.html` the entrypoint.
