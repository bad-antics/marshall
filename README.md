<div align="center">

# 🔒 Marshall

### NullSec Privacy Browser

[![License: MIT](https://img.shields.io/badge/License-MIT-red.svg?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebKit](https://img.shields.io/badge/WebKit-0088CC?style=for-the-badge&logo=safari&logoColor=white)](https://webkit.org/)
[![GTK4](https://img.shields.io/badge/GTK4-4A86CF?style=for-the-badge&logo=gtk&logoColor=white)](https://gtk.org/)
[![Version](https://img.shields.io/badge/Version-1.0.0-purple.svg?style=for-the-badge)](https://github.com/bad-antics/marshall/releases/tag/v1.0.0)
[![Release](https://img.shields.io/github/v/release/bad-antics/marshall?style=for-the-badge&color=green)](https://github.com/bad-antics/marshall/releases)

<br/>

```
███╗   ███╗ █████╗ ██████╗ ███████╗██╗  ██╗ █████╗ ██╗     ██╗     
████╗ ████║██╔══██╗██╔══██╗██╔════╝██║  ██║██╔══██╗██║     ██║     
██╔████╔██║███████║██████╔╝███████╗███████║███████║██║     ██║     
██║╚██╔╝██║██╔══██║██╔══██╗╚════██║██╔══██║██╔══██║██║     ██║     
██║ ╚═╝ ██║██║  ██║██║  ██║███████║██║  ██║██║  ██║███████╗███████╗
╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝
```

**Secure. Private. Untraceable.**

<br/>

**A hardened privacy-focused web browser built for security researchers, penetration testers, and privacy-conscious users.**

[Features](#-features) • [Installation](#-installation) • [Extensions](#-extensions) • [Usage](#-usage) • [Configuration](#️-configuration) • [Architecture](#️-architecture) • [Contributing](#-contributing)

</div>

---

## 🎯 Why Marshall?

Marshall was built from the ground up with **privacy by default**. Unlike mainstream browsers that bolt on privacy features as an afterthought, Marshall is designed to:

- **🚫 Block tracking** at every level — ads, fingerprinting, cookies, WebRTC leaks
- **🔐 Secure by default** — No telemetry, no data collection, hardened WebKit
- **🧪 OSINT-ready** — Built for security researchers with integrated recon tools
- **🧩 Extensible** — Sandboxed extensions with honeypot detection

---

## 🛡️ Features

### Privacy Protection

| Feature | Description |
|---------|-------------|
| 🚫 **Ad Blocking** | Built-in blocker with EasyList + EasyPrivacy |
| 🔒 **Tracker Blocking** | Blocks known tracking domains and scripts |
| 🎭 **Fingerprint Protection** | Prevents canvas, WebGL, and audio fingerprinting |
| 🍪 **Cookie Control** | Third-party cookies blocked by default |
| 👤 **User Agent Spoofing** | Blend in with common browser signatures |
| 🌐 **Referrer Control** | Strict referrer policy prevents leakage |
| 🔇 **WebRTC Protection** | Prevents real IP disclosure |
| 📍 **Timezone Spoofing** | Hide your real timezone |

### Security Features

| Feature | Description |
|---------|-------------|
| 🔐 **HTTPS-First** | Automatic HTTPS upgrades |
| 🛡️ **WebGL Disabled** | Prevents GPU fingerprinting by default |
| 🧅 **Tor Integration** | Optional onion routing |
| 🔐 **DNS over HTTPS** | Encrypted DNS queries via Cloudflare/Quad9 |
| 💾 **No Persistence** | Clear all data on exit (configurable) |
| 🔑 **Certificate Pinning** | HPKP support for critical sites |

### Modern Browser Experience

- **📑 Tabbed Browsing** — Full tab management with session restore
- **📚 Bookmarks** — Organize and sync your favorite sites
- **🔍 Privacy Search** — DuckDuckGo, Startpage, Searx by default
- **⌨️ Keyboard Shortcuts** — Vim-style navigation available
- **🎨 Dark Theme** — NullSec-styled interface, easy on the eyes
- **🔧 Developer Tools** — Full WebKit inspector

---

## 🔌 Extensions

Marshall supports a growing ecosystem of **sandboxed security extensions** that run in an isolated container with honeypot detection for malicious behavior.

### Featured Extensions

| Extension | Description | Language |
|-----------|-------------|----------|
| 🔍 **Shodan Lookup** | Query Shodan.io for IP/domain intelligence | JavaScript |
| 📋 **WHOIS Inspector** | Detailed domain registration info | JavaScript |
| ⚡ **XSS Scanner** | Detect Cross-Site Scripting vulnerabilities | JavaScript |
| 📝 **Header Analyzer** | HTTP security header analysis | JavaScript |
| 🔐 **Cert Inspector** | SSL/TLS certificate grading | Ruby |
| 📡 **Traffic Analyzer** | Network monitoring & anomaly detection | TypeScript |
| 🔧 **Request Tamper** | HTTP interception and modification | Lua |
| 🧠 **Memory Forensics** | Memory artifact and shellcode detection | C |

### Sandbox Security

All extensions execute in a **multi-layered sandbox**:

```
Extension → Secure Channel (AES-256-GCM) → Rust Sandbox (seccomp) → Honeypot (Go)
```

- **🦀 Rust Core** — Process isolation with seccomp-bpf and namespace separation
- **🔐 Encrypted IPC** — All communication encrypted with session keys
- **🍯 Honeypot System** — Fake services detect malicious extensions
- **📊 Threat Scoring** — Behavioral analysis triggers containment

👉 **[Browse & Install Extensions](https://github.com/bad-antics/marshall-extensions)**

---

## 📦 Installation

### Quick Install (Linux)

```bash
curl -sSL https://raw.githubusercontent.com/bad-antics/marshall/main/install.sh | bash
```

### Dependencies

<details>
<summary><b>Debian / Ubuntu</b></summary>

```bash
sudo apt install -y \
    libgtk-4-dev \
    libwebkitgtk-6.0-dev \
    pkg-config \
    libssl-dev \
    libsoup-3.0-dev
```
</details>

<details>
<summary><b>Arch Linux</b></summary>

```bash
sudo pacman -S gtk4 webkitgtk-6.0 pkg-config openssl libsoup3
```
</details>

<details>
<summary><b>Fedora</b></summary>

```bash
sudo dnf install gtk4-devel webkitgtk6.0-devel pkg-config openssl-devel libsoup3-devel
```
</details>

### From Source

```bash
git clone https://github.com/bad-antics/marshall.git
cd marshall
cargo build --release
sudo install -Dm755 target/release/marshall /usr/local/bin/marshall
```

### Pre-built Binaries

Download from [Releases](https://github.com/bad-antics/marshall/releases):
- `marshall-linux-x86_64.tar.gz`
- `marshall-linux-arm64.tar.gz`
- `.deb` and `.rpm` packages available

---

## 🚀 Usage

```bash
# Launch Marshall
marshall

# Open a specific URL
marshall https://example.com

# Private browsing mode
marshall --private

# Enable Tor routing
marshall --tor

# Maximum privacy mode
marshall --paranoid
```

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+T` | New tab |
| `Ctrl+W` | Close tab |
| `Ctrl+L` | Focus URL bar |
| `Ctrl+R` | Reload |
| `Ctrl+Shift+P` | Private window |
| `Ctrl+Shift+N` | New window |
| `Alt+←/→` | Back / Forward |
| `Ctrl++/-/0` | Zoom in/out/reset |
| `F12` | Developer tools |
| `Ctrl+F` | Find on page |
| `Ctrl+H` | History |
| `Ctrl+B` | Bookmarks |

---

## ⚙️ Configuration

**Config file:** `~/.config/marshall/config.toml`

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
spoof_user_agent = true

[adblock]
enabled = true
filter_lists = [
    "https://easylist.to/easylist/easylist.txt",
    "https://easylist.to/easylist/easyprivacy.txt",
    "https://malware-filter.gitlab.io/malware-filter/urlhaus-filter.txt"
]

[network]
tor_enabled = false
dns_over_https = true
doh_server = "https://cloudflare-dns.com/dns-query"
# Alternatives: "https://dns.quad9.net/dns-query"

[appearance]
theme = "nullsec-dark"
font_family = "JetBrains Mono"
font_size = 14

[extensions]
sandbox_level = "strict"  # minimal, standard, strict, paranoid
auto_update = true
```

---

## 🏗️ Architecture

```
marshall/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Application state
│   ├── ui/                  # GTK4 components
│   │   ├── window.rs        # Main window
│   │   ├── toolbar.rs       # Navigation bar
│   │   ├── urlbar.rs        # URL entry
│   │   ├── tabbar.rs        # Tab management
│   │   └── theme.rs         # CSS theming
│   ├── engine/              # WebKit integration
│   │   ├── webview.rs       # WebKit wrapper
│   │   └── settings.rs      # Engine config
│   ├── privacy/             # Privacy features
│   │   ├── tracker_blocker.rs
│   │   ├── fingerprint.rs
│   │   ├── cookie_manager.rs
│   │   └── referrer.rs
│   ├── adblock/             # Ad blocking
│   │   ├── engine.rs
│   │   └── lists.rs
│   ├── network/             # Network layer
│   │   ├── dns.rs           # DoH resolver
│   │   └── tor.rs           # Tor integration
│   ├── extensions/          # Extension loader
│   │   ├── manager.rs
│   │   └── sandbox.rs
│   └── config/              # Configuration
├── Cargo.toml
├── LICENSE
└── README.md
```

### Build Flags

```bash
# Standard build
cargo build --release

# With all features
cargo build --release --all-features

# Feature selection
cargo build --release --features "tor,extensions,developer"
```

| Feature | Description | Default |
|---------|-------------|---------|
| `adblock` | Ad blocking engine | ✅ |
| `extensions` | Extension support | ✅ |
| `tor` | Tor network routing | ❌ |
| `developer` | DevTools support | ❌ |

---

## 🔐 Privacy Philosophy

1. **Zero Telemetry** — No data ever leaves your machine
2. **Secure Defaults** — Privacy features enabled out of the box
3. **Transparency** — 100% open source, auditable code
4. **User Control** — You decide what data to keep or clear
5. **Defense in Depth** — Multiple layers of protection

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Clone and setup
git clone https://github.com/bad-antics/marshall.git
cd marshall

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Debug mode
RUST_LOG=debug cargo run
```

---

## 📄 License

MIT License — see [LICENSE](LICENSE)

---

## 🔗 Related Projects

| Project | Description |
|---------|-------------|
| [Marshall Extensions](https://github.com/bad-antics/marshall-extensions) | Security extensions with sandboxed execution |
| [NullSec Tools](https://github.com/bad-antics/nullsec-tools) | Comprehensive security toolkit |
| [NullSec Linux](https://nullsec.pages.dev) | Security-focused Linux distribution |

---

<div align="center">

**Built with 🦀 Rust by [bad-antics](https://github.com/bad-antics)**

*Part of the NullSec Security Suite*

[![Discord](https://img.shields.io/badge/Discord-7289DA?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/killers)
[![Website](https://img.shields.io/badge/Website-000000?style=for-the-badge&logo=About.me&logoColor=white)](https://bad-antics.github.io)

</div>
