# Astra Browser

Astra Browser is a Rust-based browser shell inspired by Firefox's UX and architecture direction, but it does not attempt to reimplement the whole Firefox engine in one pass.

Instead, it uses the native WebView stack on Linux and provides a practical starting point for a Rust browser project.

## What it does

- opens a desktop window
- loads a real web page via WebKit
- gives you a clean Rust project baseline for browser work
- is suitable for iterating toward tabs, toolbar controls, and custom browser features

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

## Linux dependencies

On Debian/Ubuntu, make sure WebKit is installed:

```bash
sudo apt install libwebkit2gtk-4.1-dev
```

## Roadmap

1. Add a Firefox-style toolbar and address bar
2. Implement tab management
3. Add browsing history and bookmarks
4. Support custom protocol and navigation hooks
5. Explore a custom engine or Gecko/Servo integration later

> This project is a browser foundation, not a complete Firefox fork. Building a full browser engine is a large project that typically takes teams of engineers years.
