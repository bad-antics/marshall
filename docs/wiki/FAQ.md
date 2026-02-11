# FAQ

## Is Marshall a fork of Firefox/Chromium?
No. Marshall is built from scratch using Rust with GTK3 for the UI and WebKitGTK for rendering. It's a completely independent browser.

## Why Rust?
Memory safety without garbage collection. Rust prevents entire classes of bugs (buffer overflows, use-after-free) that plague C/C++ browsers.

## Does Marshall support extensions?
Not yet. Extension support is planned for v2.0. Currently, privacy features are built-in.

## How does Tor integration work?
Marshall connects to a local Tor SOCKS5 proxy. You need Tor installed separately (`sudo apt install tor`). Marshall then routes selected tabs through the Tor circuit.

## Can I import bookmarks?
Yes. File > Import Bookmarks supports:
- Firefox (JSON, HTML)
- Chromium (HTML)
- Safari (HTML)
- Netscape Bookmark format

## What platforms are supported?
- Linux (primary target) — x86_64, ARM64
- macOS — experimental
- Windows — not yet supported

## How do I report a security vulnerability?
Email security@nullsec.dev or open a private security advisory on GitHub.
