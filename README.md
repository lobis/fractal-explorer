# [Fractal Explorer](https://lobis.github.io/fractal-explorer/)

[![build](https://github.com/lobis/fractal-explorer/actions/workflows/rust.yml/badge.svg)](https://github.com/lobis/fractal-explorer/actions/workflows/rust.yml)

A simple [Julia](https://en.wikipedia.org/wiki/Julia_set) and [Mandelbrot](https://en.wikipedia.org/wiki/Mandelbrot_set) Set explorer built using [Rust](https://www.rust-lang.org/). This was created as a learning project and is based on [this great tutorial](https://sotrh.github.io/learn-wgpu/).

The explorer can be run as a standalone desktop application (should work for Windows, Linux and MacOS) or as a web application for which a [demo is available](https://lobis.github.io/fractal-explorer/).

## [✨ Live Demo](https://lobis.github.io/fractal-explorer/)

## 🤓 Usage

* **Mouse Left Click**: Toggle Julia set *c* parameter selection.
* **Mouse Cursor Position**: Select Julia set _c_ parameter.
* **Mouse Wheel | Trackpad | Up & Down Arroy Keys**: Zoom on cursor position.
* **Mouse Right Click**: Hold and drag to translate fractal.
* **J | M Keys**: Toggle between Julia and Mandelbrot sets.

## ⚙️ Build

To build the executable run:

```
cargo build --release
```

There is a Windows executable available as an artifact of the v0.1.0 release which can be downloaded [here](https://github.com/lobis/fractal-explorer/releases/download/v0.1.0/fractal-explorer-app.exe). Please always be very careful when downloading executable files from the internet.

## 🌐 Web
To build the web application run the following command. You may need to install `wasm-pack` via `cargo install wasm-pack`.

```
wasm-pack build --target web --out-dir public/pkg
```

All files related to the static site will be placed under `public` with `index.html` the entrypoint. You can server the site locally using `python3 -m http.server` and then navigating to `http://localhost:8000`.

```
python -m http.server 8000 --bind 127.0.0.1 --directory public
```