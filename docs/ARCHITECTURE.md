# Marshall Architecture

## Component Overview

```
marshall/
├── src/
│   ├── main.rs          # Entry point, GTK application init
│   ├── browser.rs       # Core browser logic
│   ├── tab.rs           # Tab management
│   ├── webview.rs       # WebKitGTK wrapper
│   ├── blocker.rs       # Content blocking engine
│   ├── tor.rs           # Tor SOCKS5 integration
│   ├── privacy.rs       # Anti-fingerprint, cookie control
│   ├── ai.rs            # AI assistant sidebar
│   ├── ui/
│   │   ├── window.rs    # Main window
│   │   ├── tabbar.rs    # Tab bar widget
│   │   ├── urlbar.rs    # URL bar with autocomplete
│   │   └── sidebar.rs   # Sidebar panels
│   └── config/
│       ├── settings.rs  # User preferences
│       └── profiles.rs  # Browser profiles
├── data/
│   ├── filters/         # Ad blocking filter lists
│   └── icons/           # UI icons
└── Cargo.toml
```

## Build System

Marshall uses Cargo with feature flags:
- `default` — Core browser with ad blocking
- `tor` — Tor network integration
- `ai` — AI assistant sidebar
- `wayland` — Wayland-native rendering
