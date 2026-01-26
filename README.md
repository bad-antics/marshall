# Marshall Browser 🌐🛡️

[![GitHub stars](https://img.shields.io/github/stars/bad-antics/marshall?style=social)](https://github.com/bad-antics/marshall)
[![GitHub release](https://img.shields.io/github/v/release/bad-antics/marshall)](https://github.com/bad-antics/marshall/releases)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![GTK](https://img.shields.io/badge/GTK-3.0-green.svg)](https://gtk.org/)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)

> Privacy-focused web browser built with Rust + GTK3

## Features

- 🚫 **Built-in Ad Blocking** - Block ads and trackers by default
- 🧅 **Tor Integration** - One-click Tor circuit routing
- 🤖 **AI Assistant** - Sidebar AI for research and coding
- 🔒 **Tab Isolation** - Each tab runs in isolation
- 👤 **Anti-Fingerprinting** - Minimize browser fingerprinting
- ⚡ **Fast & Lightweight** - Native Rust performance

## Build

\`\`\`bash
git clone https://github.com/bad-antics/marshall
cd marshall
cargo build --release
./target/release/marshall
\`\`\`

## Requirements

- Rust 1.70+
- GTK 3.0
- WebKitGTK

### Debian/Ubuntu
\`\`\`bash
sudo apt install libgtk-3-dev libwebkit2gtk-4.0-dev
\`\`\`

### Arch
\`\`\`bash
sudo pacman -S gtk3 webkit2gtk
\`\`\`

## Extensions

See [Marshall Extensions](https://github.com/bad-antics/marshall-extensions) for privacy-focused browser extensions.

## Part of NullSec Suite

- [NullSec Desktop](https://github.com/bad-antics/nullsec-desktop) - Security command center
- [NullSec Linux](https://github.com/bad-antics/nullsec-linux) - Security-focused distro

## License

GPL-3.0

---
*Made by [@bad-antics](https://github.com/bad-antics)*
