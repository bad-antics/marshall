# Getting Started

## Prerequisites

| Dependency | Version | Install |
|-----------|---------|---------|
| Rust | 1.70+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| GTK 3.0 | 3.24+ | `sudo apt install libgtk-3-dev` |
| WebKitGTK | 2.38+ | `sudo apt install libwebkit2gtk-4.1-dev` |
| pkg-config | any | `sudo apt install pkg-config` |

## Build

```bash
git clone https://github.com/bad-antics/marshall
cd marshall
cargo build --release
```

## Run

```bash
./target/release/marshall
# Or with URL
./target/release/marshall https://duckduckgo.com
```

## Build Options

```bash
# Debug build (faster compile, slower runtime)
cargo build

# Release build (slower compile, optimized)
cargo build --release

# With Tor support
cargo build --release --features tor

# With AI assistant
cargo build --release --features ai
```

## Arch Linux / AUR
```bash
yay -S marshall-browser
```

## NixOS
```nix
{ pkgs, ... }: {
  environment.systemPackages = [ pkgs.marshall ];
}
```
