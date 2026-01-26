# Building from Source

## Requirements
- Rust 1.70+
- Cargo
- GTK4 development libraries
- WebKitGTK 6.0

## Clone Repository
```bash
git clone https://github.com/bad-antics/marshall
cd marshall
```

## Install Dependencies

### Debian/Ubuntu
```bash
sudo apt install \
    build-essential \
    libgtk-4-dev \
    libwebkitgtk-6.0-dev \
    libadwaita-1-dev \
    pkg-config
```

### Arch Linux
```bash
sudo pacman -S \
    base-devel \
    gtk4 \
    webkitgtk-6.0 \
    libadwaita
```

## Build

### Debug Build
```bash
cargo build
./target/debug/marshall
```

### Release Build (Optimized)
```bash
cargo build --release
./target/release/marshall
```

## Running Tests
```bash
cargo test
```

## Creating Release Package
```bash
cargo build --release
mkdir -p releases
cp target/release/marshall releases/
cd releases
tar -czvf marshall-linux-x86_64.tar.gz marshall
```

## Cross-Compilation
For other architectures, use cross:
```bash
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```
