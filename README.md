# Astra Browser

Astra Browser is a Rust-based browser shell inspired by Firefox's UX and architecture direction, but it does not attempt to reimplement the whole Firefox engine in one pass.

Instead, it uses the native WebView stack on Linux and provides a practical starting point for a Rust browser project.

## Where to keep the output
`$HOME/.local/bin/astra-browser`

## Dependencies
 - Firefox [MANDETORY]
 - Cargo [Only Necessary For Build]

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
## Install
Run the install script provided in scripts/install.sh **OR** run `git clone https://github.com/iz4c810/astra-browser` after checking your `.local/bin` folder is made with `ls ~/.local`.
now run `cd astra-browser` from the `$HOME` directory (or where ever you put the browser source) then double check `cargo` is installed with `cargo --version`, if it is then proceed to run `cargo build --release && cd target/release && cp astra-browser ~/.local/bin/astra-browser`

### IF ~/.local/bin DOESNT EXIST
`mkdir -p ~/.local/bin`

## Roadmap

1. Add a Firefox-style toolbar and address bar
2. Implement tab management
3. Add browsing history and bookmarks
4. Support custom protocol and navigation hooks
5. Explore a custom engine or Gecko/Servo integration later

> This project is a browser foundation, not a complete Firefox fork. Building a full browser engine is a large project that typically takes teams of engineers years.
