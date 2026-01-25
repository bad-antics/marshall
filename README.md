<div align="center">

# 🔒 Marshall

### NullSec Privacy Browser

[![License: MIT](https://img.shields.io/badge/License-MIT-red.svg?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebKit](https://img.shields.io/badge/WebKit-0088CC?style=for-the-badge&logo=safari&logoColor=white)](https://webkit.org/)
[![GTK3](https://img.shields.io/badge/GTK3-4A86CF?style=for-the-badge&logo=gtk&logoColor=white)](https://gtk.org/)
[![Version](https://img.shields.io/badge/Version-1.0.0-purple.svg?style=for-the-badge)](https://github.com/bad-antics/marshall/releases/tag/v1.0.0)
[![Release](https://img.shields.io/github/v/release/bad-antics/marshall?style=for-the-badge&color=green)](https://github.com/bad-antics/marshall/releases)

```
╔════════════════════════════════════════════════════════════════════════════╗
║                                                                            ║
║    ███╗   ███╗ █████╗ ██████╗ ███████╗██╗  ██╗ █████╗ ██╗     ██╗          ║
║    ████╗ ████║██╔══██╗██╔══██╗██╔════╝██║  ██║██╔══██╗██║     ██║          ║
║    ██╔████╔██║███████║██████╔╝███████╗███████║███████║██║     ██║          ║
║    ██║╚██╔╝██║██╔══██║██╔══██╗╚════██║██╔══██║██╔══██║██║     ██║          ║
║    ██║ ╚═╝ ██║██║  ██║██║  ██║███████║██║  ██║██║  ██║███████╗███████╗     ║
║    ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝     ║
║                                                                            ║
║                      Secure. Private. Untraceable.                         ║
║                                                                            ║
║                          bad-antics | 2026                                 ║
╚════════════════════════════════════════════════════════════════════════════╝
```

**A privacy-focused web browser built for security researchers and privacy-conscious users.**

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Configuration](#configuration) • [Building](#building)

</div>

---

## 🛡️ Features

### Privacy Protection
- **🚫 Ad Blocking** — Built-in ad blocker using EasyList and EasyPrivacy
- **🔒 Tracker Blocking** — Blocks known tracking domains and scripts
- **🎭 Fingerprint Protection** — Prevents browser fingerprinting techniques
- **🍪 Cookie Control** — Block third-party cookies by default
- **👤 User Agent Spoofing** — Blend in with common browsers
- **🌐 Referrer Control** — Strict referrer policy to prevent leakage

### Security Features
- **🔐 HTTPS-First** — Prefer secure connections
- **🛡️ WebGL Disabled** — Prevents GPU fingerprinting
- **📍 Timezone Spoofing** — Hide your real timezone
- **🔇 WebRTC Protection** — Prevents IP leakage
- **💾 No Persistence** — Clear all data on exit (optional)

### Modern Browser Features
- **📑 Tabbed Browsing** — Full tab management
- **📚 Bookmarks** — Organize your favorite sites
- **🔍 Smart Search** — DuckDuckGo by default
- **⌨️ Keyboard Shortcuts** — Efficient navigation
- **🎨 Dark Theme** — NullSec-styled dark interface

### Optional Features
- **🧅 Tor Integration** — Route traffic through Tor network
- **🔐 DNS over HTTPS** — Encrypted DNS queries
- **🔧 Developer Tools** — Web inspector for developers


### 🔌 Extensions
Marshall supports installable extensions for enhanced functionality:

- **🔍 Shodan Lookup** — Query Shodan.io for IP/domain intelligence
- **📋 WHOIS Inspector** — Detailed domain registration info
- **⚡ XSS Scanner** — Detect Cross-Site Scripting vulnerabilities
- **📝 Header Analyzer** — Security header analysis
- **🔐 Cert Inspector** — SSL/TLS certificate grading
- **📡 Traffic Analyzer** — Network monitoring and anomaly detection
- **🔧 Request Tamper** — HTTP interception and modification

👉 **[Browse all extensions](https://github.com/bad-antics/marshall-extensions)**
---

## 📦 Installation

### Dependencies (Debian/Ubuntu)

```bash
sudo apt install -y \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    pkg-config \
    libssl-dev
```

### Dependencies (Arch Linux)

```bash
sudo pacman -S gtk3 webkit2gtk pkg-config openssl
```

### Dependencies (Fedora)

```bash
sudo dnf install gtk3-devel webkit2gtk4.0-devel pkg-config openssl-devel
```

### From Source

```bash
# Clone repository
git clone https://github.com/bad-antics/marshall.git
cd marshall

# Build release
cargo build --release

# Install
sudo install -Dm755 target/release/marshall /usr/local/bin/marshall
```

### From Releases

Download pre-built binaries from the [Releases](https://github.com/bad-antics/marshall/releases) page.

---

## 🚀 Usage

```bash
# Start Marshall
marshall

# Open specific URL
marshall https://duckduckgo.com

# Private window mode
marshall --private

# With Tor enabled (requires tor feature)
marshall --tor
```

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+T` | New tab |
| `Ctrl+W` | Close tab |
| `Ctrl+L` | Focus URL bar |
| `Ctrl+R` | Reload page |
| `Ctrl+Shift+P` | New private window |
| `Alt+Left` | Go back |
| `Alt+Right` | Go forward |
| `Ctrl++` | Zoom in |
| `Ctrl+-` | Zoom out |
| `Ctrl+0` | Reset zoom |
| `F12` | Developer tools |
| `Ctrl+F` | Find on page |

---

## ⚙️ Configuration

Configuration file: `~/.config/marshall/config.toml`

```toml
[general]
homepage = "about:blank"
search_engine = "https://duckduckgo.com/?q="
restore_session = false
enable_javascript = true
enable_webgl = false

[privacy]
strict_mode = true
block_trackers = true
block_fingerprinting = true
block_third_party_cookies = true
clear_on_exit = true
do_not_track = true
spoof_timezone = true

[adblock]
enabled = true
filter_lists = [
    "https://easylist.to/easylist/easylist.txt",
    "https://easylist.to/easylist/easyprivacy.txt"
]

[appearance]
theme = "nullsec-dark"
dark_mode = true
font_family = "JetBrains Mono"
font_size = 14

[network]
tor_enabled = false
dns_over_https = true
doh_server = "https://cloudflare-dns.com/dns-query"
```

---

## 🔧 Building

### Standard Build

```bash
cargo build --release
```

### With All Features

```bash
cargo build --release --features "tor,developer"
```

### Feature Flags

| Feature | Description |
|---------|-------------|
| `adblock` | Ad blocking (default) |
| `tor` | Tor network integration |
| `developer` | Developer tools |

---

## 🏗️ Architecture

```
marshall/
├── src/
│   ├── main.rs           # Application entry point
│   ├── ui/               # GTK4 UI components
│   │   ├── window.rs     # Main browser window
│   │   ├── toolbar.rs    # Navigation toolbar
│   │   ├── urlbar.rs     # URL/address bar
│   │   ├── tabbar.rs     # Tab management UI
│   │   ├── statusbar.rs  # Status bar
│   │   └── theme.rs      # CSS theming
│   ├── engine/           # WebKit engine wrapper
│   ├── privacy/          # Privacy protection
│   │   ├── tracker_blocker.rs
│   │   ├── fingerprint_protection.rs
│   │   └── cookie_manager.rs
│   ├── adblock/          # Ad blocking engine
│   ├── tabs/             # Tab management
│   ├── bookmarks/        # Bookmark storage
│   ├── history/          # Browsing history
│   ├── config/           # Configuration
│   └── network/          # Network utilities
├── Cargo.toml
├── LICENSE
└── README.md
```

---

## 🔐 Privacy Philosophy

Marshall is built on the principle of **privacy by default**:

1. **Minimal Data Collection** — We don't collect any telemetry
2. **Secure Defaults** — Privacy features enabled out of the box
3. **Transparency** — Open source, auditable code
4. **User Control** — You decide what data to keep

---

## 🤝 Contributing

Contributions welcome! Please read our contributing guidelines.

```bash
# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file.

---

## 🔗 Links

- **Extensions:** [marshall-extensions](https://github.com/bad-antics/marshall-extensions)

- **Website:** [bad-antics.github.io](https://bad-antics.github.io)
- **Discord:** [discord.gg/killers](https://discord.gg/killers)
- **GitHub:** [@bad-antics](https://github.com/bad-antics)

---

<div align="center">

**Built with 🦀 Rust by bad-antics**

*Part of the NullSec Security Suite*

</div>
