# Installation

## Pre-built Binary
Download from [Releases](https://github.com/bad-antics/marshall/releases)

```bash
tar -xzvf marshall-linux-x86_64.tar.gz
./marshall
```

## Dependencies (Debian/Ubuntu)
```bash
sudo apt install libgtk-4-dev libwebkitgtk-6.0-dev libadwaita-1-dev
```

## Dependencies (Arch)
```bash
sudo pacman -S gtk4 webkitgtk-6.0 libadwaita
```

## Building from Source
```bash
git clone https://github.com/bad-antics/marshall
cd marshall
cargo build --release
./target/release/marshall
```
